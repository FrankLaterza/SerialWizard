import {invoke} from "@tauri-apps/api/tauri";
import {Dropdown} from "@nextui-org/react";

export default function About() {
    async function hanndleChangeBaud() {
        let data = await invoke("baud", {});
        // console.log(data);
        // const newLines : any = [...lines, data];
        // setLines(newLines);
        // invoke("greet", {name: "World"}).then(console.log).catch(console.error);
    }

    return (
        <main className=" overflow-y-autoflex gap-4 justify-start items-center flex-col w-screen min-h-screen h-fill bg-gray-800">
            About
        </main>
    );
}
