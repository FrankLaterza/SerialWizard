import Image from "next/image";
import { Inter } from "next/font/google";
import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FiSend } from "react-icons/fi";
import { Dropdown } from "@nextui-org/react";
import { type } from "os";
import { disconnect } from "process";
import eta from "public/eta_space.png"

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
    const [lines, setLines] = useState<string[]>([]);
    const [inputValueText, setInputValueText] = useState("");

    const handleInputChangeTextBox = (event: any) => {
        setInputValueText(event.target.value);
    };

    async function hanndleHello() {
        let data = await invoke("greet", { name: "World" });
        console.log(data);
        const newLines: any = [...lines, data];
        setLines(newLines);
    }

    function writeNewLines(str: String) {
        const newLines: any = [...lines, str];
        setLines(newLines);
    }

    const [isConnected, setIsConnected] = useState(false);

    async function handleConnect() {
        // i don't know how to free the port in rust
        console.log("connecting");
        // get number from set<string>
        const baud = parseInt(Array.from(selectedBaud).join(""));
        let res;
        // set the baud
        await invoke("set_baud", { boardRate: baud });
        // get string from set<string>
        const port = Array.from(selectedPort).join("");
        // set the port
        await invoke("set_port", { portName: port });
        // open serial
        let data = await invoke("open_serial", {});
        console.log(data);
        if (!data) {
            writeNewLines("failed to start serial port");
            setIsConnected(false);
        }
        else {
            writeNewLines("connection successful");
            setIsConnected(true);
        }
    }

    async function handleDisconnect() {
        setIsConnected(false);
        let data = await invoke("close_port", {});
    }

    async function handleSend(event: any) {
        event.preventDefault();
        const newLines: any = [...lines, inputValueText];
        setLines(newLines);
        setInputValueText("");
        console.log(inputValueText);
        let data = await invoke("send_serial", { input: inputValueText });
        if (!data) {
            writeNewLines("failed to send serial");
        }
    }

    const [inputValuePower, setInputValuePower] = useState("");

    const handleInputChangePower = (event: any) => {
        setInputValuePower(event.target.value);
    };
    const [inputValueStop, setInputValueStop] = useState("");

    const handleInputChangeStop = (event: any) => {
        setInputValueStop(event.target.value);
    };

    const scrollRef = useRef<HTMLDivElement>(null);

    const scrollToBottom = () => {
        if (scrollRef.current) {
            scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
        }
    };

    useEffect(() => {
        scrollToBottom();
    }, [lines, lines.length]);

    async function handleWriteInput(str: String) {
        setInputValuePower("");
        setInputValueStop("");
        const newLines: any = [...lines, str];
        setLines(newLines);
        await invoke("send_serial", { input: str });
        console.log(str);
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
        if (data.length !== 0) {
            const portItems = data.map((portName: any) => ({ name: portName }));
            setPortItems(portItems);
        }
        // setPortItems(data);
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
        <main className="flex justify-start items-center flex-col w-screen overflow-hidden h-screen min-h-screen bg-gray-800">
            {/* header */}
            <div className="flex flex-row gap-10 justify-center items-center w-full h-[10%] py-4 text-xl text-center bg-gray-900">
                Cryo Interface
                <Image
                    src={eta}
                    width={150}
                    height={150}
                    alt="Picture of the author"
                />
            </div>
            {/* main message box */}
            <div className="h-[77%] w-full gap-2 p-5 flex flex-row">
                {/* buttons left*/}
                <div className="w-3/6 flex flex-col items-center">
                    <div className="bg-gray-900 rounded-xl w-full p-2 text-xl text-center mb-2">
                        Custom Buttons
                    </div>
                    {/* buttons container */}
                    <div className="bg-gray-700 rounded-xl border-4 border-gray-600 flex flex-wrap h-full w-full justify-center items-center p-6  gap-x-6 gap-y-3">
                        <div className="rounded-lg gap-2 bg-gray-400 p-3 w-[50%] h-[20%] flex flex-row items-center justify-center">
                            <button
                                onClick={() => handleWriteInput(`SET PWOUT=${inputValuePower}`)}
                                className="border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                            >
                                Set Power
                            </button>
                            <input
                                id="myInput"
                                type="text"
                                className="w-[40%] h-1/2 text-sm text-black border-2 border-gray-700 p-2"
                                value={inputValuePower}
                                onChange={handleInputChangePower}
                            />
                        </div>

                        <button
                            onClick={() => handleWriteInput("SET PWOUT")}
                            className="h-[20%] w-[40%] border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                        >
                            Print Set Power
                        </button>

                        <div className="rounded-lg gap-2 bg-gray-400 p-3 w-[50%] h-[20%] flex flex-row items-center justify-center">
                            <button
                                onClick={() => handleWriteInput(`SET SSTOP=${inputValueStop}`)}
                                className="border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                            >
                                Set Stop Status
                            </button>
                            <input
                                id="myInput"
                                type="text"
                                className="w-[40%] h-1/2 text-sm text-black border-2 border-gray-700 p-2"
                                value={inputValueStop}
                                onChange={handleInputChangeStop}
                            />
                        </div>

                        <button
                            onClick={() => handleWriteInput("SET SSTOP")}
                            className="h-[20%] w-[40%] border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                        >
                            Print Stop Status
                        </button>
                        <button
                            onClick={() => handleWriteInput("P")}
                            className="h-[20%] w-[40%] border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                        >
                            Print Power
                        </button>
                        <button
                            onClick={() => handleWriteInput("E")}
                            className="h-[20%] w-[40%] border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                        >
                            Print Error
                        </button>
                        <button
                            onClick={() => handleWriteInput("TC")}
                            className="h-[20%] w-[40%] border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                        >
                            Print Cold Tip Temp
                        </button>
                    </div>
                </div>
                {/* message box and text box right */}
                <div className="w-full h-full flex flex-col">
                    {/* message box */}
                    <div ref={scrollRef} className="overflow-y-scroll resize-none h-full flex flex-grow justify-start flex-col bg-gray-500">
                        <div className="flex-1 p-4">
                            {lines.map((line, index) => (
                                <p key={index}>{line}</p>
                            ))}
                        </div>
                    </div>
                    {/* text box */}
                    <form onSubmit={handleSend} className="flex flex-row items-center w-full">
                        <input
                            id="myInput"
                            type="text"
                            className="w-full text-black border-2 border-gray-400 p-3 w-full"
                            value={inputValueText}
                            onChange={handleInputChangeTextBox}
                        />
                        <FiSend
                            onClick={handleSend}
                            className="color-white bg-blue-800 h-full w-14 p-3"
                        />
                    </form>
                </div>
            </div>
            {/* bottom buttons */}
            <div className="w-5/6 mb-5 h-[10%] py-4 flex justify-around items-center flex-row text-xl text-center rounded-lg bg-gray-900 ">
                <button
                    // onClick={isConnected ? handleDisconnect : handleConnect}
                    onClick={handleConnect}
                    className="border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 font-bold py-2 px-4 rounded-lg"
                >
                    {isConnected ? "Connected" : "Connect"}
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
                        </Dropdown.Menu>
                    </Dropdown>
                </div>
                <div className="flex flex-row justify-center items-center gap-4 border border-gray-400 bg-gray-600 text-gray-200 font-bold py-2 px-4 rounded-lg">
                    Port:
                    <Dropdown disableAnimation>
                        <Dropdown.Button
                            flat
                            color="primary"
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
                        </Dropdown.Menu>
                    </Dropdown>
                </div>
            </div>
        </main>
    );
}
