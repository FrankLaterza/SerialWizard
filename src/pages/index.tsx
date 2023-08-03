import Image from "next/image";
import { Inter, ZCOOL_QingKe_HuangYou } from "next/font/google";
import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FiSend, FiPlus, FiMinus } from "react-icons/fi";
import { Dropdown } from "@nextui-org/react";
import { type } from "os";
import { disconnect } from "process";
import eta from "public/eta_space.png"

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
    const [lines, setLines] = useState<String[]>([]);
    const [inputValueText, setInputValueText] = useState("");



    type MessageType = {
        message: string;
        type: string;
    };
    const [messageBox, setMessageBox] = useState<MessageType[]>([]);

    function getColorByType(type: String) {
        switch (type) {
            case "CONSOLE":
                return "#000000";
            case "SEND":
                return "red";
            case "RECEIVE":
                return "white";
            default:
                return "white";
        }
    }

    function messages() {
        return (
            <div>
              {messageBox.map(({ message, type }, index) => (
                <div key={index} style={{ color: getColorByType(type) }}>
                  {message}
                </div>
              ))}
            </div>
          );
    };

    const writeNewLines = (str: string, type: string) => {
        setMessageBox((prevMessageBox) => [...prevMessageBox, { message: str, type }]);
      };

    const handleInputChangeTextBox = (event: any) => {
        setInputValueText(event.target.value);
    };

    async function handleHello() {
        let data = await invoke("greet", { name: "World" });
        console.log(data);
        const newLines: any = [...lines, data];
        setLines(newLines);
    }

    const [isDropped, setIsDropped] = useState(true);

    function handleDropToggle() {
        setIsDropped(!isDropped);
    }

    const [isConnected, setIsConnected] = useState(false);

    async function handleConnect() {
        // i don't know how to free the port in rust
        // i guess i'll just connect to nothing and force an err
        // if connected then connect to nothing to disconnect
        if (isConnected) {
            console.log(isConnected);
            setIsConnected(false);
            await invoke("set_baud", { boardRate: 0 });
            await invoke("set_port", { portName: "" });
            await invoke("open_serial", {});
            writeNewLines("\ndisconnection successful\n", "CONSOLE");
            return;
        }

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
            writeNewLines("\nfailed to start serial port\n", "CONSOLE");
            setIsConnected(false);
        }
        else {
            writeNewLines("\nconnection successful\n", "CONSOLE");
            setIsConnected(true);
        }
    }

    async function handleDisconnect() {
        setIsConnected(false);
        let data = await invoke("close_port", {});
    }

    const [delimiter, setDelimiter] = useState("\n");

    async function handleSend(event: any) {
        event.preventDefault();
        writeNewLines(inputValueText + "\n", "SEND");
        setInputValueText("");

        let data = await invoke("send_serial", { input: inputValueText + getEnding() });
        if (!data) {
            writeNewLines("\nfailed to send serial\n", "CONSOLE");
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
    }, [messageBox, messageBox.length]);

    async function handleWriteCustom(str: string) {
        setInputValuePower("");
        setInputValueStop("");
        const newLines: any = [...lines, str];
        writeNewLines(str + "\n", "SEND");

        let data = invoke("send_serial", { input: str + getEnding() });
        if (!data) {
            writeNewLines("\nfailed to send serial\n", "CONSOLE");
        }
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
        // console.log(portItems);
    }

    const [selectedBaud, setSelectedBaud] = useState<any>(new Set(["9600"]));

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

    async function update_serial() {
        let data: string = await invoke("receive_update", {}); // fix type any
        // console.log(data)
        if (data !== "") {
            console.log(data);
            writeNewLines(data, "RECEIVE");
        }
    }

    const [ending, setEnding] = useState<any>(new Set(["\\n\\r"]));

    const endingItems = [
        { name: "\\n\\r" },
        { name: "\\n" },
        { name: "\\r" },
        { name: "none" }
    ];

    function getEnding() {
        console.log("ending: " + endingItems.find(item => ending.has(item.name))?.name);
        switch (true) {
            case ending.has("\\n\\r"):
                return "\n\r";
            case ending.has("\\n"):
                return "\n";
            case ending.has("\\r"):
                return "\r";
            case ending.has("none"):
                return "";
        }
    }

    useEffect(() => {
        const intervalId = setInterval(update_serial, 25);
        return () => clearInterval(intervalId);
    }, []);

    const optionButtonContainerStyle = `h-[80px] lg:h-[20%] lgh:w-[90%] w-[90%] lg:w-[45%] rounded-lg gap-1 bg-gray-400 p-1 flex flex-row items-center justify-center`;
    const optionButtonStyle = `border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 text-sm lg:text-base font-bold py-2 px-2 rounded-lg`;
    const singleButtonStyle = `h-[80px] lg:h-[20%] lgh:w-[90%] w-[90%] lg:w-[45%] border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 text-sm lg:text-base font-bold py-2 px-2 rounded-lg`;

    return (
        <main className="flex justify-between flex-col gap-0 w-screen overflow-hidden h-screen min-h-screen bg-gray-800">
            {/* header */}
            <div className="flex flex-row gap-10 px-10 justify-between items-center w-full h-100 py-4 text-xl text-center bg-gray-900">
                {(isDropped) ?
                    <FiMinus
                        onClick={handleDropToggle}
                        className="hover:bg-blue-500 color-white bg-blue-800 h-14 w-14 rounded-full p-3"
                    />
                    :
                    <FiPlus
                        onClick={handleDropToggle}
                        className="hover:bg-blue-500 color-white bg-blue-800 h-14 w-14 rounded-full p-3"
                    />
                }
                <div>
                    Serial Monitor
                </div>
                <Image
                    src={eta}
                    width={150}
                    height={150}
                    alt="Picture of the author"
                />

            </div>
            {/* main message box */}
            <div className="h-full w-full gap-2 p-5 flex flex-row overflow-hidden">
                {/* buttons left*/}
                {(isDropped) ?
                    <div className="w-3/6 flex flex-col items-center">
                        <div className="bg-gray-900 rounded-xl w-full p-2 text-xl text-center mb-2">
                            Custom Buttons
                        </div>
                        {/* buttons container */}
                        <div className="overflow-y-scroll bg-gray-700 rounded-xl border-4 border-gray-600 flex flex-wrap h-full w-full justify-center items-center p-2  gap-x-6 gap-y-3">
                            <div className={optionButtonContainerStyle}>
                                <button
                                    onClick={() => handleWriteCustom(`SET PWOUT=${inputValuePower}`)}
                                    className={optionButtonStyle}
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
                                onClick={() => handleWriteCustom("SET PWOUT")}
                                className={singleButtonStyle}
                            >
                                Print Set Power
                            </button>

                            <div className={optionButtonContainerStyle}>
                                <button
                                    onClick={() => handleWriteCustom(`SET SSTOP=${inputValueStop}`)}
                                    className={optionButtonStyle}
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
                                onClick={() => handleWriteCustom("SET SSTOP")}
                                className={singleButtonStyle}
                            >
                                Print Stop Status
                            </button>
                            <button
                                onClick={() => handleWriteCustom("P")}
                                className={singleButtonStyle}
                            >
                                Print Power
                            </button>
                            <button
                                onClick={() => handleWriteCustom("E")}
                                className={singleButtonStyle}
                            >
                                Print Error
                            </button>
                            <button
                                onClick={() => handleWriteCustom("TC")}
                                className={singleButtonStyle}
                            >
                                Print Cold Tip Temp
                            </button>
                        </div>
                    </div>
                    : ""}
                {/* message box and text box right */}
                <div className="w-full h-full flex flex-col ">
                    {/* message box */}
                    <div ref={scrollRef} className="overflow-y-scroll resize-none h-full flex flex-grow justify-start flex-col bg-gray-500">
                        <div className="flex-1 p-4">
                            {messages()}
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
            <div className="w-full h-[12%] flex justify-around items-center flex-row text-xl text-center rounded-lg bg-gray-900 ">
                <button
                    onClick={handleConnect}
                    className="border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 text-sm lg:text-base font-bold py-2 px-4 rounded-lg"
                >
                    {isConnected ? "Disconnect" : "Connect"}
                </button>
                {/* ending dropdown */}
                <div className="flex flex-row justify-center items-center gap-4 border border-gray-400 bg-gray-600 text-gray-200 text-sm lg:text-base font-bold py-2 px-4 rounded-lg">
                    Ending:
                    <Dropdown disableAnimation>
                        <Dropdown.Button
                            flat
                            color="primary"
                        >
                            {ending}
                        </Dropdown.Button>
                        <Dropdown.Menu
                            aria-label="Multiple selection actions"
                            color="secondary"
                            disallowEmptySelection
                            selectionMode="single"
                            selectedKeys={ending}
                            items={endingItems}
                            onSelectionChange={setEnding}
                        >

                            {endingItems.map(endingItems => (
                                <Dropdown.Item
                                    key={endingItems.name}
                                    color={"default"}
                                >
                                    {endingItems.name}
                                </Dropdown.Item>
                            ))}
                        </Dropdown.Menu>
                    </Dropdown>
                </div>
                {/* baud dropdown */}
                <div className="flex flex-row justify-center items-center gap-4 border border-gray-400 bg-gray-600 text-gray-200 text-sm lg:text-base font-bold py-2 px-4 rounded-lg">
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
                <div className="flex flex-row justify-center items-center gap-4 border border-gray-400 bg-gray-600 text-gray-200 text-sm lg:text-base font-bold py-2 px-4 rounded-lg">
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
