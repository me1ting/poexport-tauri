import { useLoaderData } from "react-router-dom";
import { Input, Label, Button, useToastController, } from "@fluentui/react-components";
import { Save16Regular, Save16Filled, bundleIcon } from "@fluentui/react-icons";

import './settings.css';
import { TOASTER_ID, notifyError, notifySuccess } from "./common/notify";
import { Config } from "../cmds/types";
import { getConfigCmd, setPoeSessIdCmd } from "../cmds/cmds";

const Save = bundleIcon(Save16Filled, Save16Regular);

export async function loader() {
    const conf = await getConfigCmd()
    return { conf }
}

export default function Settings() {
    const { conf } = useLoaderData() as { conf: Config };

    const toastController = useToastController(TOASTER_ID);

    function changePoeSessId() {
        const poeSessId: string = (document.getElementById("poeSessId") as HTMLInputElement).value;
        setPoeSessIdCmd(poeSessId).then(() => {
            notifySuccess(toastController, "已保存");
        }).catch(err => {
            notifyError(toastController, err);
        })
    }

    return <div>
        <section className="section">
            <div className="line">
                <span className="line-left"><Label htmlFor="poeSessId">PoeSessId</Label></span>
                <span className="line-right" style={{ display: "flex", alignItems: "center" }}>
                    <Input id="poeSessId" size="small" appearance="outline" defaultValue={conf.poeSessId} />
                    <Button icon={<Save />} appearance="subtle" shape="circular" className="edit" onClick={changePoeSessId} />
                </span>
            </div>
        </section>
    </div>
}