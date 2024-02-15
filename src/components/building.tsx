import { Button, Input, Select, useToastController, Text } from "@fluentui/react-components";
import './building.css';
import { useId, useState } from "react";
import { TOASTER_ID, notifyError, notifySuccess } from "./common/notify";
import { Character } from "../poe/types";
import Assets from "cn-poe-export-db";
import { BasicTranslatorFactory } from "cn-poe-translator";
import { transform } from "pob-building-creater";
import pako from "pako";
import { getCharactersCmd, getItemsCmd, getPassiveSkillsCmd } from "../cmds/cmds";

const factory = new BasicTranslatorFactory(Assets);
const translator = factory.getJsonTranslator();

function createBuilding(items: any, passiveSkills: any): string {
    translator.translateItems(items);
    translator.translatePassiveSkills(passiveSkills);
    const building = transform(items, passiveSkills).toString();
    const compressed = pako.deflate(building.toString());
    const code = btoa(String.fromCharCode(...compressed))
        .replaceAll("+", "-")
        .replaceAll("/", "_");
    return code;
}

export default function Building() {
    const [accountName, setAccountName] = useState<string>("");
    const [characters, setCharacters] = useState<Character[]>([]);
    const [league, setLeague] = useState<string>("");
    const [character, setCharacter] = useState<string>("");
    const [code, setCode] = useState<string>("");
    const toastController = useToastController(TOASTER_ID);

    const leagues = characters.map(c => c.league)
        .filter((value, index, array) => { return array.indexOf(value) === index })
        .map(league => <option key={league}>{league}</option>);

    const shownCharacters = characters.filter(value => value.league == league)
        .map(c => <option key={c.name}>{c.name}</option>);

    const loadAccount = async () => {
        const inputAccountName = (document.getElementById("accountName") as HTMLInputElement).value;
        let data = "";
        try {
            data = await getCharactersCmd(inputAccountName);
        }catch(err){
            notifyError(toastController, `${err}`);
            return;
        }
        const characters = JSON.parse(data) as Character[];
        setCharacters(characters);
        if (characters.length > 0) {
            setLeague(characters[0].league);
            setCharacter(characters[0].name);
        }
        notifySuccess(toastController, "已读取");
    }

    const leagueChangedHandler = (e: any) => {
        const league = e.target.value;
        setLeague(league);
        const first = characters.find(c => c.league == league);
        if (first === undefined) {
            setCharacter("");
        } else {
            setCharacter(first.name);
        }
    }

    const exportBuilding = async () => {
        setCode("");
        let data = "";
        try {
            data = await getItemsCmd(accountName, character);
        }catch(err){
            notifyError(toastController, `${err}`);
            return;
        }
        const items = JSON.parse(data);
        
        try{
            data = await getPassiveSkillsCmd(accountName, character);
        }catch(err){
            notifyError(toastController, `${err}`);
            return;
        }

        const passiveSkills = JSON.parse(data);

        setCode(createBuilding(items, passiveSkills));
        notifySuccess(toastController, "已导出");
    }

    const copyCode = () => {
        navigator.clipboard.writeText(code);
        notifySuccess(toastController, "已复制");
    }

    const leagueSelectorId = useId();
    const characterSelectorId = useId();

    return <div>
        <div className="section">
            <div className="line">
                <Input id="accountName" value={accountName} className="main-input" onChange={(e) => { setAccountName(e.target.value); setCharacters([]) }} placeholder="论坛账户名" />
                <Button onClick={loadAccount} disabled={accountName.length == 0}>读取</Button>
            </div>
            <div className="line" style={{ justifyContent: "normal" }}>
                <Select id={leagueSelectorId} disabled={leagues.length == 0} className="main-input" onChange={leagueChangedHandler}>{leagues}</Select>
            </div>
            <div className="line" style={{ justifyContent: "normal" }}>
                <Select id={characterSelectorId} disabled={shownCharacters.length == 0} className="main-input" onChange={(e) => setCharacter(e.target.value)}>{shownCharacters}</Select>
            </div>
            <div className="line">
                <Button appearance="primary" onClick={exportBuilding} disabled={characters.length == 0}>导出</Button>
            </div>
            <div className="line">
                <Text wrap={false} truncate style={{ width: "250px" }}>{code}</Text>
                <Button disabled={code.length === 0} onClick={copyCode}>复制</Button>
            </div>
        </div>
    </div>
}