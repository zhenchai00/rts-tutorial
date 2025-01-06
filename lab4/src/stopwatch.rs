use std::{thread, time::Duration};
use std::sync::mpsc::channel;
use scheduled_thread_pool::ScheduledThreadPool;
pub fn stopwatch(){
    // create channels
    let (s_tx, s_rx) = channel();
    let (m_tx, m_rx) = channel();

    let pool = ScheduledThreadPool::new(2);

    // schedule the seconds task
    let mut sec = 0;
    pool.execute_at_fixed_rate(
        Duration::from_secs(0), 
        Duration::from_secs(1), 
        move || {
            s_tx.send(sec).unwrap();
            sec += 1;
            if sec == 60{
                sec = 0;
            }
        }
    );

    let mut min = 0;
    pool.execute_at_fixed_rate(
        Duration::from_secs(0),
        Duration::from_secs(60), 
        move || {
            m_tx.send(min).unwrap();
            min += 1;
        }
    );

    // spawn seconds thread
    // thread::spawn(move ||{
    //     let mut sec = 0;
    //     loop{
    //         s_tx.send(sec).unwrap();
    //         sec+=1;
    //         if sec ==60{
    //             sec= 0;
    //         }
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // minutes thread
    // thread::spawn(move ||{
    //     let mut min = 0;
    //     loop{
    //         m_tx.send(min).unwrap();
    //         min +=1;
    //         thread::sleep(Duration::from_secs(60));
    //     }
    // });
    // main thread receives and prints

    let mut current_mins = 0;
    loop{
        let s = s_rx.recv().unwrap(); // block
        match m_rx.try_recv() {
            Ok(data) => current_mins = data,
            Err(_) => {}
        } 
        println!("{current_mins}:{s}");
    }
}