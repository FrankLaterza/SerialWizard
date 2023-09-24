import Image from "next/image";
import { Inter, ZCOOL_QingKe_HuangYou } from "next/font/google";
import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Header from "../components/header"
import MessageBox from "../components/message_box"
import BasicMenu from "../components/menu"

export default function Home() {

    // handles dropped menu status
    const [isDropped, setIsDropped] = useState(true);

    // makes a window from rust
    async function hanndleSetup() {
        await invoke("make_window", {});
    }

    return (
        <main className="flex justify-between flex-col gap-0 w-screen overflow-hidden h-screen min-h-screen bg-gray-800">
            <BasicMenu/>
            <Header isDropped={isDropped} setIsDropped={setIsDropped} />
            <MessageBox isDropped={isDropped} />
        </main>
    );
}
