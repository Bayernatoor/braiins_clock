use crate::helpers::startup; 
use crate::requests::{send_to_blockclock::*, self}; 

pub async fn program_loop() -> () {
    let mut tags = startup::select_tags();
    
    let refresh_rate = startup::refresh_rate();
    
    loop {
        let active_tag = &tags.remove(0);
        tags.push(active_tag.to_string());

        let url_to_send = requests::send_to_blockclock::clock_tags_url(active_tag.to_string(), None).await;
   
        let request = requests::send_to_blockclock::send_to_blockclock(url_to_send).await;
        

        //for t in &tags {
        //     println!("{}", t);
        //};
        //println!("active_tag, {:?}", active_tag);
        //for t in &tags {
        //    println!("after {}", t);
        //};
        break;
    }
   //return refresh_rate 

}
