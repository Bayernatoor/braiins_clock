use crate::helpers::startup; 
use crate::requests::{send_to_blockclock::*, self}; 

pub async fn program_loop() -> () {
    let mut tags = startup::select_tags();
    
    let refresh_rate = startup::refresh_rate();
    
    loop {
        let active_tag = &tags.remove(0);
        tags.push(active_tag.to_string());

        let symbol = select_symbol(active_tag);
        println!("TAG: {:?}", active_tag);
        println!("Symbol: {:?}", symbol);
    

        //let url_to_send = requests::send_to_blockclock::clock_tags_url(active_tag.to_string()).await;
        let url_to_send = requests::send_to_blockclock::slush_tags_url(active_tag.to_string(), Some(symbol)).await;
   
        let request = requests::send_to_blockclock::send_to_blockclock(url_to_send).await;

        println!("RESPONSE: {:?}", request);
        

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
