use std::{
    sync::Arc,
    thread,
    time::Duration,
    sync::atomic::{AtomicBool, Ordering}
};

// Вот это всё объяснить в документации
pub fn run_mut_task<F>(mut task: F, run: Arc<AtomicBool>)
where
    F: FnMut() + Send + 'static,
{
    thread::spawn(move || {
        while run.load(Ordering::SeqCst) {
            task();
            thread::sleep(Duration::from_secs(1));
        }
        println!("\nsystem::run_mut_task ended succesefully\n");
    });
}
