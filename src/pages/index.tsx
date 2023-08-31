import Image from "next/image";
import {Inter, ZCOOL_QingKe_HuangYou} from "next/font/google";
import {useState, useRef, useEffect} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {emit, listen} from "@tauri-apps/api/event";
import {FiSend, FiPlus, FiMinus} from "react-icons/fi";
import {Dropdown} from "@nextui-org/react";
import {type} from "os";
import {disconnect} from "process";
import eta from "public/eta_space.png";
import {BsFillPauseFill, BsPlayFill} from "react-icons/bs";
import {FaFolder} from "react-icons/fa";

const inter = Inter({subsets: ["latin"]});

export default function Home() {
    const [lines, setLines] = useState<String[]>([]);
    const [inputValueText, setInputValueText] = useState("");
    const [messageBox, setMessageBox] = useState<String>("");

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
            <div className="flex-1 p-4">
                {messageBox.split("\n").map((line, index) => (
                    <div key={index}>{line}</div>
                ))}
            </div>
        );
    }

    function writeNewLines(str: string) {
        setMessageBox((messageBox) => messageBox.concat(str));
    }

    const handleInputChangeTextBox = (event: any) => {
        setInputValueText(event.target.value);
    };

    async function handleHello() {
        let data = await invoke("greet", {name: "World"});
        console.log(data);
        const newLines: any = [...lines, data];
        setLines(newLines);
    }

    const [isDropped, setIsDropped] = useState(true);

    function handleDropToggle() {
        setIsDropped(!isDropped);
    }

    function DropDownButton() {
        return (
            <>
                {isDropped ? (
                    <FiMinus
                        onClick={handleDropToggle}
                        className="hover:bg-blue-500 color-white bg-blue-800 h-12 w-12 rounded-full p-3"
                    />
                ) : (
                    <FiPlus
                        onClick={handleDropToggle}
                        className="hover:bg-blue-500 color-white bg-blue-800 h-12 w-12 rounded-full p-3"
                    />
                )}
            </>
        );
    }

    const [isRecording, setIsRecording] = useState(true);

    function handleRecordToggle() {
        if (isRecording) {
            writeNewLines("\n(Serial console) Recording Started\n");
            setIsRecording(false);
        } else {
            writeNewLines("\n(Serial console) Recording Saved\n");
            setIsRecording(true);
        }
    }

    function RecordButton() {
        return (
            <>
                {isRecording ? (
                    <div className="flex flex-row items-center gap-2">
                        Rec
                        <BsFillPauseFill
                            onClick={handleRecordToggle}
                            className="hover:bg-red-500 color-white bg-red-700 h-12 w-12 rounded-full p-3"
                        />
                    </div>
                ) : (
                    <div className="flex flex-row items-center gap-2">
                        Rec
                        <BsPlayFill
                            onClick={handleRecordToggle}
                            className="hover:bg-red-500 color-red bg-red-700 h-12 w-12 rounded-full p-3"
                        />
                    </div>
                )}
            </>
        );
    }

    async function handleFolderSet() {
        let data = await invoke("print_file_path");
        if (data) {
            writeNewLines("\n(Serial console) Path Set Successful\n");
        } else {
            writeNewLines("\n(Serial console) Path Set Unsuccessful\n");
        }
    }

    function FolderButton() {
        return (
            <div className="flex flex-row items-center gap-2">
                <FaFolder
                    onClick={handleFolderSet}
                    className="hover:bg-gray-300 color-white bg-gray-500 h-12 w-12 rounded-full p-3"
                />
            </div>
        );
    }

    const [delimiter, setDelimiter] = useState("\n");

    async function handleSend(event: any) {
        event.preventDefault();
        // writeNewLines(inputValueText + "\n");
        setInputValueText("");

        let data = await invoke("send_serial", {input: inputValueText});
        if (!data) {
            writeNewLines("\n(Serial console) Failed to send serial\n");
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
        let data = invoke("send_serial", {input: str});
        if (!data) {
            writeNewLines("\n(Serial console) Failed to send serial\n");
        }
    }
    // makes a window from rust
    async function hanndleSetup() {
        await invoke("make_window", {});
    }

    useEffect(() => {
        startSerialEventListener();
    }, []);

    // same as payload
    type Payload = {
        message: string;
    };

    async function startSerialEventListener() {
        await listen<Payload>("updateSerial", (event: any) => {
            writeNewLines(event.payload.message);
        });
    }

    const optionButtonContainerStyle = `h-[80px] lg:h-[20%] lgh:w-[90%] w-[90%] lg:w-[45%] rounded-lg gap-1 bg-gray-400 p-1 flex flex-row items-center justify-center`;
    const optionButtonStyle = `border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 text-sm lg:text-base font-bold py-2 px-2 rounded-lg`;
    const singleButtonStyle = `h-[80px] lg:h-[20%] lgh:w-[90%] w-[90%] lg:w-[45%] border border-gray-400 bg-gray-600 hover:bg-gray-400 hover:text-white text-gray-200 text-sm lg:text-base font-bold py-2 px-2 rounded-lg`;

    return (
        <main className="flex justify-between flex-col gap-0 w-screen overflow-hidden h-screen min-h-screen bg-gray-800">
            {/* header */}
            <div className="flex flex-row px-10 justify-center items-center w-full h-100 py-4 text-xl text-center bg-gray-900">
                <div className="flex justify-start w-1/3 gap-10">
                    <DropDownButton />
                </div>
                <div className="flex justify-center w-1/3 text-center">
                    Serial Monitor
                </div>
                <div className="flex justify-end w-1/3">
                    <Image
                        src={eta}
                        width={150}
                        height={150}
                        alt="Picture of the author"
                    />
                </div>
            </div>
            {/* main message box */}
            <div className="h-full w-full gap-2 p-5 flex flex-row overflow-hidden">
                {/* buttons left*/}
                {isDropped ? (
                    <div className="w-3/6 flex flex-col items-center">
                        <div className="bg-gray-900 rounded-xl w-full p-2 text-xl text-center mb-2">
                            Custom Buttons
                        </div>
                        {/* buttons container */}
                        <div className="overflow-y-scroll bg-gray-700 rounded-xl border-4 border-gray-600 flex flex-wrap h-full w-full justify-center items-center p-2  gap-x-6 gap-y-3">
                            <div className={optionButtonContainerStyle}>
                                <button
                                    onClick={() =>
                                        handleWriteCustom(
                                            `SET PWOUT=${inputValuePower}`
                                        )
                                    }
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
                                    onClick={() =>
                                        handleWriteCustom(
                                            `SET SSTOP=${inputValueStop}`
                                        )
                                    }
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
                ) : (
                    ""
                )}
                {/* message box and text box right */}
                <div className="overflow-hidden w-full h-full flex flex-col">
                    {/* message box */}
                    <div
                        ref={scrollRef}
                        className="overflow-y-scroll resize-none h-full flex flex-grow justify-start flex-col bg-gray-500"
                    >
                        {messages()}
                    </div>
                    {/* text box */}
                    <form
                        onSubmit={handleSend}
                        className="flex flex-row items-center w-full"
                    >
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
        </main>
    );
}
