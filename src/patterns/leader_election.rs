use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct LeaderElection {
    id: u32,
    nodes: Arc<Vec<u32>>,
    leader: Arc<Mutex<Option<u32>>>,
}

impl LeaderElection {
    async fn start(self) {
        let (tx, mut rx) = mpsc::channel(10);

        while let Some(id) = self.nodes.get(self.id as usize).cloned() {
            let current_leader = self.leader.lock().await.clone();

            match current_leader {
                Some(leader) => {
                    if leader == id {
                        println!("I, {} am already the leader.", id);
                    } else {
                        if tx.send(leader).await.is_err() {
                            println!("Leader {} is no longer available.", leader);
                            self.leader.lock().await.replace(id);
                            println!("I, {}  am the new leader.", id);
                        } else {
                            println!("Leader {} is still alive.", leader);
                        }
                    }
                }
                None => {
                    self.leader.lock().await.replace(id);
                    println!("I, {} am the new leader.", id);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            while let Some(leader) = rx.recv().await {
                if leader == self.leader.lock().await.unwrap() {
                    self.leader.lock().await.take();
                    break;
                }
            }
        }
    }
}


#[cfg(test)]
mod async_tests {
    use std::sync::{Arc};
    use tokio::sync::{Mutex};

    use tokio::join;
    use crate::patterns::leader_election::LeaderElection;


    #[tokio::test()]
    async fn async_test3() {
        let nodes = Arc::new(vec![1, 2, 3, 4]);
        let leader = Arc::new(Mutex::new(None));
    
        let election1 = LeaderElection {
            id: 0,
            nodes: nodes.clone(),
            leader: leader.clone(),
        };
        let election2 = LeaderElection {
            id: 1,
            nodes: nodes.clone(),
            leader: leader.clone(),
        };
        let election3 = LeaderElection {
            id: 2,
            nodes: nodes.clone(),
            leader: leader.clone(),
        };
        let election4 = LeaderElection {
            id: 3,
            nodes: nodes.clone(),
            leader: leader.clone(),
        };
    
        join!(
            election1.start(),
            election2.start(),
            election3.start(),
            election4.start(),
        );

        let assigned_leader = leader.clone().lock().await.unwrap();
        assert_eq!(assigned_leader, 1);
    }

}

