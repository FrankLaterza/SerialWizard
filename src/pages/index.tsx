import Image from "next/image";
import { Inter } from "next/font/google";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FiSend } from "react-icons/fi";
import { Dropdown } from "@nextui-org/react";
import { type } from "os";
import { disconnect } from "process";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
    const [lines, setLines] = useState<string[]>(["hello", "world"]);
    const [inputValue, setInputValue] = useState("");

    const handleInputChange = (event: any) => {
        setInputValue(event.target.value);
    };

    async function hanndleHello() {
        let data = await invoke("greet", { name: "World" });
        console.log(data);
        const newLines: any = [...lines, data];
        setLines(newLines);
    }

    const [isConnected, setIsConnected] = useState(false);

    async function handleConnect() {
        // i don't know how to free the port in rust
        setIsConnected(false);
        console.log("connecting");
        // get number from set<string>
        const baud = parseInt(Array.from(selectedBaud).join(""));
        // set the baud
        await invoke("set_baud", { boardRate: baud });
        // get string from set<string>
        const port = Array.from(selectedPort).join("");
        // set the port
        await invoke("set_port", { portName: port });
        let data = await invoke("open_serial", {});
    }

    async function handleDisconnect() {
        setIsConnected(false);
        let data = await invoke("close_port", {});
    }

    async function handleSend() {
        setInputValue("");
        await invoke("send_serial", { input: inputValue });
    }

    // makes a window from rust
    async function hanndleSetup() {
        await invoke("make_window", {});
    }

    type PortItem = {
        name: string;
        // add other properties here if necessary
    };

    const [selectedPort, setSelectedPort] = useState<any>(new Set(["select"]));
    const [portItems, setPortItems] = useState<PortItem[]>([
        { name: "no ports" },
    ]);

    async function handleGetPorts() {
        // gets the ports
        let data: any = await invoke("get_ports", {}); // fix type any
        // setPortItems(data);
        const portItems = data.map((portName: any) => ({ name: portName }));
        setPortItems(portItems);
        console.log(portItems);
    }

    const [selectedBaud, setSelectedBaud] = useState<any>(new Set(["115200"]));

    const baudItems = [
        { name: "300" },
        { name: "1200" },
        { name: "2400" },
        { name: "4800" },
        { name: "9600" },
        { name: "19200" },
        { name: "38400" },
        { name: "57600" },
        { name: "74880" },
        { name: "115200" },
        { name: "230400" },
        { name: "250000" },
        { name: "500000" },
        { name: "1000000" },
        { name: "2000000" },
    ];
    async function handleSetBaud() {
        // gets the ports
        console.log(parseInt(Array.from(selectedBaud).join("")));
        // const baud = parseInt(selectedBaud)
        // let data: any = await invoke("set_baud", {parseInt("9600")}); // fix type any
        // const portItems = data.map((portName: any) => ({ name: portName }));
        // // setPortItems(data);
        // console.log(portItems);
    }

    async function update_serial() {
        let data: any = await invoke("receive_update", {}); // fix type any
        // console.log(data)
        if (data !== "") {
            console.log(data);
            setLines(lines => [...lines, data]);
        }
    }
    useEffect(() => {
        const intervalId = setInterval(update_serial, 1000);
        return () => clearInterval(intervalId);
    }, []);

    return (
        <main className="flex justify-center items-center flex-col w-screen h-screen min-h-screen overflow-hidden bg-gray-800">
            {/* header */}
            <div className="w-full h-fit py-4 text-xl text-center bg-gray-900">
                Serial Monitor
            </div>
            {/* message box */}
            <div className="w-4/6 h-full overflow-y-scroll mt-5 flex justify-center flex-col bg-gray-500">
                <div className="flex-1 p-4">
                    {lines.map((line, index) => (
                        <p key={index}>{line}</p>
                    ))}
                </div>
            </div>
            {/* text box */}
            <div className="flex flex-row items-center w-4/6">
                <input
                    id="myInput"
                    type="text"
                    className="w-full text-black border-2 border-gray-400 p-3 w-full"
                    value={inputValue}
                    onChange={handleInputChange}
                />
                <FiSend
                    onClick={handleSend}
                    className="color-white bg-blue-800 h-full w-14 p-3"
                />
            </div>
            {/* bottom buttons */}
            <div className="w-4/6 my-6 h-fit py-4 flex justify-around items-center flex-row text-xl text-center rounded-lg bg-gray-900 ">
                <button
                    onClick={isConnected ? handleDisconnect : handleConnect}
                    className="border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                >
                    {isConnected ? "Disconnect" : "Connect"}
                </button>

                <button
                    onClick={handleSend}
                    className="border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                >
                    Send
                </button>
                {/* baud dropdown */}
                <div className="flex flex-row justify-center items-center gap-4 border border-gray-400 bg-gray-600 text-gray-200 font-bold py-2 px-4 rounded-lg">
                    Buad:
                    <Dropdown disableAnimation>
                        <Dropdown.Button
                            flat
                            color="primary"
                            css={{ tt: "capitalize" }}
                        >
                            {selectedBaud}
                        </Dropdown.Button>
                        <Dropdown.Menu
                            aria-label="Multiple selection actions"
                            color="secondary"
                            disallowEmptySelection
                            selectionMode="single"
                            selectedKeys={selectedBaud}
                            items={baudItems}
                            onSelectionChange={setSelectedBaud}
                        >
                            {baudItems.map(baudItems => (
                                <Dropdown.Item
                                    key={baudItems.name}
                                    color={"default"}
                                >
                                    {baudItems.name}
                                </Dropdown.Item>
                            ))}
                            {/* <Dropdown.Item key="text">Text</Dropdown.Item>
                        <Dropdown.Item key="number">Number</Dropdown.Item>
                        <Dropdown.Item key="date">Date</Dropdown.Item>
                        <Dropdown.Item key="single_date">
                            Single Date
                        </Dropdown.Item>
                        <Dropdown.Item key="iteration">Iteration</Dropdown.Item> */}
                        </Dropdown.Menu>
                    </Dropdown>
                </div>
                <div className="flex flex-row justify-center items-center gap-4 border border-gray-400 bg-gray-600 text-gray-200 font-bold py-2 px-4 rounded-lg">
                    Port:
                    <Dropdown disableAnimation>
                        <Dropdown.Button
                            flat
                            color="primary"
                            css={{ tt: "capitalize" }}
                            onPress={handleGetPorts}
                        >
                            {selectedPort}
                        </Dropdown.Button>
                        <Dropdown.Menu
                            aria-label="Multiple selection actions"
                            color="secondary"
                            disallowEmptySelection
                            selectionMode="single"
                            selectedKeys={selectedPort}
                            items={portItems}
                            onSelectionChange={setSelectedPort}
                        >
                            {portItems.map((portItem) => (
                                <Dropdown.Item
                                    key={portItem.name}
                                    color="default"
                                >
                                    {portItem.name}
                                </Dropdown.Item>
                            ))}
                            {/* <Dropdown.Item key="text">Text</Dropdown.Item>
                        <Dropdown.Item key="number">Number</Dropdown.Item>
                        <Dropdown.Item key="date">Date</Dropdown.Item>
                        <Dropdown.Item key="single_date">
                            Single Date
                        </Dropdown.Item>
                        <Dropdown.Item key="iteration">Iteration</Dropdown.Item> */}
                        </Dropdown.Menu>
                    </Dropdown>
                </div>
            </div>
        </main>
    );
}
