#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

//use tauri::async_runtime::Mutex;
use std::sync::Mutex;
use std::{collections::HashMap, env::current_exe};
use serde::{Serialize,Deserialize};
use std::io::prelude::*;

//use std::path::{Path, PathBuf};
//use tauri::api::dir::*;

use rand::Rng;

/// カードの中身
/// 
#[derive(Debug, Serialize,Deserialize, Clone)]
struct Card(
  String,
  bool,
  Vec<u128>,
);


/// カード一覧
/// 
/// id → Card
#[derive(Debug, Serialize)] //, Serialize)]
pub struct CardDeck {
deck: Mutex<HashMap<u128,Card>>,
}
impl CardDeck{
pub fn new() -> Self{
  Self { deck: Mutex::new(
    HashMap::<u128,Card>::new()
  ) }
}
}


/// カードの並べ方
///  Vec の Vec
/// これは仮かな
struct Ids(Mutex<Vec<Vec<u128>>>);
impl Ids{

  fn new()->Self{
      let v = vec![0u128;0];
      let v = vec![v];
      Self(
          Mutex::new(
              v
          )
      )
  }
}

#[tauri::command]
fn get_ids(
ids: tauri::State<'_,Ids>,
)->String{
let mut ids = ids.0.lock().unwrap();

let ids = ids[0].clone();
let ids = u1282str(ids);
return serde_json::to_string(&ids).unwrap();

}

#[tauri::command]
fn get_txts(
ids: tauri::State<'_,Ids>,
cd: tauri::State<'_, CardDeck>,
)->String{
let mut ids = ids.0.lock().unwrap();
let ids = ids[0].clone();
//let ids = u1282str(ids);

let mut cd = cd.deck.lock().unwrap();

let mut texts = Vec::<String>::new(); // = vec![String;0];
for id in ids{
  texts.push(cd.get(&id).unwrap().0.clone());
}


return serde_json::to_string(&texts).unwrap();

}

/// input1
/// 単純にvectorの最後に追加する方法
/// push をすればOk
#[tauri::command]
fn input(
text: String,
cd : tauri::State<'_,CardDeck>,
ids : tauri::State<'_,Ids>,
){

let mut id = rand::thread_rng();
let id:u128 = id.gen();

println!("{}",id);

let mut ids = ids.0.lock().unwrap();
ids[0].push(id);

let mut cd = cd.deck.lock().unwrap();

let c = Card(text,false,vec![0;0]);
if ! cd.contains_key(&id){
  cd.insert(id, c);
}
println!("{:?}",cd);
}

/// insert1
/// 挿入する方法
#[tauri::command]
async fn insert(
  id1: u128,
  id2: u128,
){ 
// 交換
let a = ();
}

/// rm
#[tauri::command]
async fn rm(){

}



/// 出力する文字列を取得する
/// 
#[tauri::command]
async fn getlist()->String{
 return "are".to_string();
} 

/// save
/// 
#[tauri::command]
fn save(
ids: tauri::State<'_,Ids>,
cd: tauri::State<'_, CardDeck>,
){
let mut jsonpath = current_exe().unwrap();
jsonpath.push("../data/json.json");
let mut idspath = current_exe().unwrap();
idspath.push("../data/ids.json");

let cd = cd.deck.lock().unwrap().clone();
let cd = serde_json::to_string(&cd).unwrap();

let mut fo = std::fs::File::create(jsonpath).unwrap();
write!(fo, "{}", cd);

let ids = ids.0.lock().unwrap().clone();
let ids = serde_json::to_string(&ids).unwrap().clone();
let mut fo = std::fs::File::create(idspath).unwrap();

write!(fo, "{}", ids);



}

/// load
/// 
#[tauri::command]
fn load(
ids: tauri::State<'_,Ids>,
cd: tauri::State<'_, CardDeck>,
){
let mut jsonpath = current_exe().unwrap();
jsonpath.push("../data/json.json");
let mut idspath = current_exe().unwrap();
idspath.push("../data/ids.json");

let mut cd = cd.deck.lock().unwrap();
let carddeck = std::fs::read_to_string(jsonpath).unwrap();
*cd = serde_json::from_str(&carddeck).unwrap();


let idinput = std::fs::read_to_string(idspath).unwrap();

let mut ids = ids.0.lock().unwrap();
*ids = serde_json::from_str(&idinput).unwrap();
}

#[tauri::command]
async fn getpath()-> String{
let mut dirpath = current_exe().unwrap();
dirpath.push("../data");

println!("いってるよね?\n{:?}", dirpath);
return dirpath.to_str().unwrap().to_string();
}

fn u1282str (ids: Vec<u128>) -> Vec<String>{
let mut v = Vec::<String>::new();
for id in ids{
  v.push(id.to_string());//.unwrap());
}
v
}

/// save するファイルが存在してなかったら作る
fn ファイル確認(){
let mut dirpath = current_exe().unwrap();
dirpath.push("../data");
//let dirpath = dirpath.

if !dirpath.is_dir(){
  std::fs::create_dir_all(&dirpath);
}

println!("{:?}", dirpath);

let mut jsonpath = dirpath.clone();
let mut textpath = dirpath.clone();

jsonpath.push("json.json");
textpath.push("text.txt");

if ! jsonpath.is_file(){
  std::fs::File::create(&jsonpath);
}

if ! textpath.is_file(){
  std::fs::File::create(textpath);
}

}



fn main() {
ファイル確認();

  tauri::Builder::default()
    .manage(Ids::new())
    .manage(CardDeck::new())
      .invoke_handler(tauri::generate_handler![
          save, load,
          input, insert,
          getpath,
          get_ids, get_txts,
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
