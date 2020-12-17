// use std::option::Option;
// struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }
//     fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }

//     fn request_preview(&mut self){
//         if let Some(s) = self.state.take(){
//             self.state = Some(s.request_preview());
//         }
//     }

//     fn approve(&mut self){
//         if let Some(s) = self.state.take(){
//             self.state = Some(s.approve());
//         }
//     }
// }

// trait State {
//     fn request_preview(self:Box<Self>)->Box<dyn State>;
//     fn approve(self:Box<Self>)->Box<dyn State>;
//     fn content<'a>(&self,post :&'a Post)->&'a str{
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     fn request_preview(self:Box<Self>)->Box<dyn State> {
//         Box::new(PendingRreview{})
//     }

//     fn approve(self:Box<Self>)->Box<dyn State> {
//         self
//     }
// }
// struct PendingRreview{}

// impl State for PendingRreview {
//     fn request_preview(self:Box<Self>)->Box<dyn State>{
//         self
//     }
    
//     fn approve(self:Box<Self>)->Box<dyn State>{
//         Box::new(Published{})
//     }
// }
// struct Published {}

// impl State for Published {

//     fn request_preview(self:Box<Self>)->Box<dyn State>{
//         self
//     }
//     fn approve(self:Box<Self>)->Box<dyn State>{
//         self
//     }

//     fn content<'a>(&self,post:&'a Post)->&'a str{
//         &post.content
//     }

// }

// #[test]
// fn test_oop() {
//     let mut post = Post::new();
//     post.add_text("I ate a salad for lunch today!");
// }
