import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const Field = (textlist:string[], idlist:string[]):JSX.Element[]=>{
  return (textlist.map((moji,idx)=>{
    return(
    <div 
    className="Card"
    key={idlist[idx]}>
      {moji}
    </div>
    )
  }))
}

function TextListField(
    prop: {timing:string}
  ):JSX.Element{

  const timing = prop.timing;
  const [textlist, setTextlist] = useState<string[]>(
    ["A", "B"]
    );
  const [idlist, setIdlist] = useState<string[]>(
    ["1","2"]
    );

    useEffect(()=>{
      gettextlist();
    },[timing])
  


  async function gettextlist(){
    let ids:string =  await invoke("get_ids");
    let txts:string = await invoke("get_txts");
    console.log(ids);
    console.log(txts);

    let ids2: string[] = JSON.parse(ids);
    let txts2: string[] = JSON.parse(txts);
    //console.log(ids);

    setTextlist(txts2);
    setIdlist(ids2);

  }
  

  return (
    <div
      className="CardField"
    >
      {Field(textlist,idlist)}
   {/*    <button
        onClick={()=>gettextlist()}
      >
        更新
      </button> */}
    </div>
  );
}

function TextField(
  prop: {text:string}
){
  let text = prop.text;

  return(
    <div>
      {text}
    </div>
  )
}


function App(){
  const [text, setText] = useState<string>("");
  //const [ids,setIds] = useState<string[]>([]);
  const [filepath, setFilepath] = useState("");
  const [textlist, setTextlist] = useState<string[]>([]);
  const [timing,setTiming] = useState<string>("");
 
/* 
  async function getpath(){
    invoke("getpath")
    console.log(path);
    setFilepath(path);
  }

  */
 useEffect(()=>{
    getpath()
  },[])

  async function getpath(){
    let are:string = await invoke("getpath");
    //console.log(are);
    setFilepath(are);
  }


  function append(){
    invoke("input", {text: text});
    setTiming(text);
    setText("");
  }

  function save(){
    invoke("save");
  }

  function load(){
    invoke("load");
    setTiming("LOAD");
  }

  function appendkey(e: React.KeyboardEvent){
    if (e.shiftKey){
      if (e.key=="Enter"){
        console.log("成功");
        if(text.match(/^[\s\n]*$/)){
          setText("")
          return
        }
        append();
        return;
      }
    } else if(e.ctrlKey){
      if (e.key == "s"){
        save();
        return;
      } else if (e.key=="o"){
        load();
        return;
      }
    }
    
  }

  function areaClean(){
    if (text.match(/^[\s\n]*$/)){
      setText("") 
    }
  }

  return (
    <div className="container">
      <div>
        <textarea
          value={text}
          onChange={(e)=>setText(e.target.value)}
          onKeyDown={(e)=>appendkey(e)}
          onKeyUp={()=>areaClean()}
        />
        <button
          onClick={()=>append()}
        >
          append (shift + Enter)
        </button>
        <button
          onClick={()=>save()}
        >
          SAVE (Ctrl+S)
        </button>
        <button
          onClick={()=>load()}
        >
          LOAD (Ctrl+O)
        </button>
      </div>      
      {/*<div>
        {text}
      </div>
  */}
      {/* <TextField text={text}/> */}
      <TextListField timing={timing} />

      <div>
        {filepath}
      </div>

    </div>
  );
}

export default App;
