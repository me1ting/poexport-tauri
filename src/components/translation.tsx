import { Link } from "react-router-dom";
import { open } from '@tauri-apps/api/shell';

const goTo = (e: { preventDefault: () => void; })=>{
    e.preventDefault();
    open("https://poe.pathof.top/item");
}

export default function Translation(){
    return <div className="section"><div className="line"><Link to={"https://poe.pathof.top/item"} onClick={goTo}>poe.pathof.top/item</Link></div></div>
}