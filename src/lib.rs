use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Node {
    addr: AccountId,
    node_type: u8, // 0: GPU, 1: CPU
    tasks_completed: u64,
    compute_time_contributed: u64,
    reputation_score: u64,
    uptime: u64,
    last_active: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Cluster {
    head_node_id: Option<u64>,
    cluster_type: u8, // 0: Ray cluster, 1: Mega Ray Cluster
    cluster_processor: u8, // 0: GTX, 1: GeforceGTX, 2: Nvidia
    nodes: Vector<Node>,
    location: String,
    tasks: Vector<Task>,
    task_ids: Vector<u64>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Task {
    creator: AccountId,
    cluster_processor: u8,
    cluster_type: u64,
    reward_amount: u64,
    completed: bool,
    completed_by: Option<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Network {
    clusters: Vector<Cluster>,
    nodes: Vector<Node>,
    tasks: Vector<Task>,
    total_compute_time: u64,
    reward_per_compute_unit: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    network: Network,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            network: Network {
                clusters: Vector::new(b"c"),
                nodes: Vector::new(b"n"),
                tasks: Vector::new(b"t"),
                total_compute_time: 0,
                reward_per_compute_unit: 100,
            },
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn init(&mut self) {
        self.network = Network {
            clusters: Vector::new(b"c"),
            nodes: Vector::new(b"n"),
            tasks: Vector::new(b"t"),
            total_compute_time: 0,
            reward_per_compute_unit: 100,
        };
    }

    // === Mutation Functions ===

    pub fn register_cluster(&mut self, cluster_type: u8, cluster_processor: u8, location: String) {
        let account_id = env::signer_account_id();

        let cluster = Cluster {
            head_node_id: None,
            cluster_type,
            cluster_processor,
            nodes: Vector::new(b"n"),
            location,
            tasks: Vector::new(b"t"),
            task_ids: Vector::new(b"i"),
        };

        self.network.clusters.push(&cluster);
    }

    pub fn submit_task(
        &mut self,
        cluster_id: u64,
        cluster_processor: u8,
        cluster_type: u64,
        reward: u64,
    ) {
        let account_id = env::signer_account_id();
        let mut cluster = self.network.clusters.get(cluster_id).expect("Cluster not found");

        let task = Task {
            creator: account_id.clone(),
            cluster_processor,
            cluster_type,
            reward_amount: reward,
            completed: false,
            completed_by: None,
        };

        self.network.tasks.push(&task);
        cluster.tasks.push(&task);
        cluster.task_ids.push(&(self.network.tasks.len() as u64 - 1));
    }

    pub fn complete_task(&mut self, task_index: u64, compute_time: u64) {
        let account_id = env::signer_account_id();
        let mut task = self.network.tasks.get(task_index).expect("Task not found");
        assert!(!task.completed, "Task already completed");

        let node_index = self.network.nodes.iter().position(|node| node.addr == account_id).expect("Node not found");
        let mut node = self.network.nodes.get(node_index as u64).expect("Node not found");

        node.tasks_completed += 1;
        node.compute_time_contributed += compute_time;

        task.completed = true;
        task.completed_by = Some(account_id.clone());

        self.network.total_compute_time += compute_time;
    }

    // === View Functions ===

    pub fn query_task_info(&self, task_index: u64) -> (AccountId, bool, Option<AccountId>) {
        let task = self.network.tasks.get(task_index).expect("Task not found");
        (task.creator.clone(), task.completed, task.completed_by.clone())
    }

    pub fn query_clusters_id(&self) -> Vec<u64> {
        (0..self.network.clusters.len()).collect()
    }

    pub fn query_cluster_tasks(&self, cluster_id: u64) -> Vec<u64> {
        let cluster = self.network.clusters.get(cluster_id).expect("Cluster not found");
        cluster.task_ids.to_vec()
    }

    pub fn query_tasks(&self, account_id: AccountId) -> Vec<u64> {
        self.network.tasks.iter().enumerate()
            .filter(|(_, task)| task.creator == account_id)
            .map(|(index, _)| index as u64)
            .collect()
    }
}