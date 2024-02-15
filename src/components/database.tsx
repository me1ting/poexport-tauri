import { Link } from "react-router-dom";
import { open } from '@tauri-apps/api/shell';

const goTo = (e: { preventDefault: () => void; })=>{
    e.preventDefault();
    open("https://poe.pathof.top/query");
}

export default function Database(){
    return <div className="section"><div className="line"><Link to={""} onClick={goTo}>poe.pathof.top/query</Link></div></div>
}