import { invoke } from "@tauri-apps/api/tauri";

async function handleGetPorts(setPorts: any) {
  const ports = await invoke("get_ports", {});
  setPorts(ports);
}

async function handleConnect(port: string, baud: string, ending: string, setIsConnected: any) {
  ending = convertEnding(ending)
  const isConnected = await invoke("handle_serial_connect", {port, baud, ending});
  setIsConnected(isConnected);
}

function getBaudList() { 
  return [
    "300",
    "1200",
    "2400",
    "4800",
    "9600",
    "19200",
    "38400",
    "57600",
    "74880",
    "115200",
    "230400",
    "250000",
    "500000",
    "1000000",
    "2000000",
  ];
}

function getEnding() {
  return [
    "None",
    "\\n",
    "\\r",
    "\\n\\r"
  ]
}

function convertEnding(ending: string) {
  switch (ending) {
    case "None":
      return "";
    case "\\n":
      return "\n";
    case "\\r":
      return "\r";
    case "\\n\\r":
      return "\n\r";
    default:
      return ""; // Default to an empty string if the label is not recognized
  }
}

async function handle_record(setIsRecording: any) {
  const res = await invoke("handle_start_record", {});
  setIsRecording(res);
}

async function handle_set_folder() {
  await invoke("set_folder_path", {});
}
export { handleGetPorts, handleConnect, handle_record, handle_set_folder, getBaudList, getEnding } 