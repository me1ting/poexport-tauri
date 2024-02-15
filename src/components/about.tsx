import { Link, Spinner, } from "@fluentui/react-components";
import { useLoaderData } from "react-router-dom";
import { UpdateInfo } from "../cmds/types";
import { open } from '@tauri-apps/api/shell';
import { getVersion } from '@tauri-apps/api/app';
import { useEffect, useState } from "react";
import { checkUpdateCmd } from "../cmds/cmds";

export async function loader() {
    const appVersion = await getVersion();

    const updateInfo: UpdateInfo = {
        needUpdate: false,
        current: appVersion,
        latest: "",
        changelog: "",
    };
    return { updateInfo }
}

type InfoStatus = "init" | "checking" | "success" | "failure";

export default function About() {
    const info = (useLoaderData() as { updateInfo: UpdateInfo }).updateInfo;
    const [updateInfo, setUpdateInfo] = useState(info);
    const [infoStatus, setInfoStatus] = useState("init" as InfoStatus);

    useEffect(() => {
        setInfoStatus("checking");
        checkUpdateCmd().then((info) => {
            setUpdateInfo(info);
            setInfoStatus("success");
        }).catch(() => {
            setInfoStatus("failure");
        });
    }, []);

    const githubClickHandler = (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        open("https://github.com/cn-poe-community/poexport");
    }

    const panClickHandler = (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        open("https://www.lanzout.com/b02vcj9hg");
    }

    const latestShow = function () {
        if (infoStatus === "init" || infoStatus === "checking") {
            return <Spinner size="extra-tiny" labelPosition="after" label="检查更新..." />;
        } else if (infoStatus === "success") {
            return <>{updateInfo.latest}</>;
        } else if (infoStatus === "failure") {
            return <>检查更新失败</>;
        }

        return <></>;
    }();

    return <div className="section">
        <div className="line"><span>当前版本</span><span>{updateInfo.current}</span></div>
        <div className="line"><span>最新版本</span><span>{latestShow}</span></div>
        <div className="line"><span>下载地址</span><span>
            <Link onClick={githubClickHandler}>github</Link>
            <Link style={{ marginLeft: "10px" }} onClick={panClickHandler}>网盘（密码1234）</Link>
        </span></div>
    </div>
}