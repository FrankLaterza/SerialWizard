import React from 'react';
import { FaFile, FaCog, FaMinus, FaExpand, FaTimes } from 'react-icons/fa';
import { useState, useRef, useEffect } from "react";
import { handleGetPorts, getBaudList, handleConnect, getEnding, handle_record, handle_set_folder } from "../utils/serial"

function MenuItem({ text, onClick}: any) {
  return <li onClick={onClick} className="px-6 py-2 bg-white hover:bg-gray-400 cursor-pointer text-black">{text}</li>;
}

function SubMenu({ text, setHook, menuItemList }: any) {
  const [isSubOpen, setIsSubOpen] = useState(false);

  const openDropdown = () => {
    setIsSubOpen(true);
  };

  const closeDropdown = () => {
    setIsSubOpen(false);
  };

  function handleSelection (item: string){
    setHook(item)
  }

  return (
    <li
      className="relative"
      onMouseEnter={openDropdown}
      onMouseLeave={closeDropdown}
    >
      <li className="px-6 py-2 cursor-pointer text-black bg-white hover:bg-gray-400">{text}</li>
      {isSubOpen && (
        <ul className="absolute w-max bg-white left-full top-0">
          {menuItemList.map((item: any, index: any) => (
              <MenuItem key={index} text={item} onClick={() => handleSelection(item)}/>
          ))}
        </ul>
      )}
    </li>
  );
}

function Serial() {
  const [baud, setBaud] = useState("9600");
  const [port, setPort] = useState("None");
  const [portList, setPortList] = useState(["None"]);
  const [ending, setEnding] = useState("None");
  const [isConnected, setIsConnected] = useState(false);
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);

  // open dropdown and also gets dynamic data
  function openDropdown() {
    setIsDropdownOpen(true);
    handleGetPorts(setPortList);
  };

  function closeDropdown() {
    setIsDropdownOpen(false);
  };

  return (
    <nav
      onMouseEnter={openDropdown}
      onMouseLeave={closeDropdown}
    >
      <ul className="flex justify-center items-center py-2 cursor-pointer">
        <li className="relative h-fit px-4">
          <span>Serial</span>
          {isDropdownOpen && (
            <ul className="flex flex-col w-max absolute bg-white my-2 block">
              <MenuItem text={isConnected ? "Disconnect" : "Connect"} onClick={() => handleConnect(port, baud, ending, setIsConnected)} />
              <SubMenu
                text={`Baud: ${baud}`}
                setHook={setBaud}
                menuItemList={getBaudList()}
              />
              <SubMenu
                text={`Port: ${port}`}
                setHook={setPort}
                menuItemList={portList}
              />
              <SubMenu
                text={`Ending: ${ending}`}
                setHook={setEnding}
                menuItemList={getEnding()}
              />
            </ul>
          )}
        </li>
      </ul>
    </nav>
  );
}

function Record(){ 
    const [isDropdownOpen, setIsDropdownOpen] = useState(false);
    const [isRecording, setIsRecording] = useState(false);

    const openDropdown = () => {
      setIsDropdownOpen(true);
    };
  
    const closeDropdown = () => {
      setIsDropdownOpen(false);
    };
  
    return (
      <nav
        onMouseEnter={openDropdown}
        onMouseLeave={closeDropdown}
      >
        <ul className="flex justify-center items-center py-2 cursor-pointer">
          <li className="relative h-fit px-4">
            <span>Record</span>
            {isDropdownOpen && (
              <ul className="flex flex-col w-max absolute bg-white my-2 block">
                <MenuItem text={ isRecording ? "Stop" : "Record"} onClick={() => handle_record(setIsRecording)} />
                <MenuItem text="Set Folder" onClick={() => handle_set_folder()} />
              </ul>
            )}
          </li>
        </ul>
      </nav>
    );
}

export default function WindowBar() {

  // TODO move function to backend
  async function closeWindow() {
    const appWindow = (await import('@tauri-apps/api/window')).appWindow
    appWindow.close()
  }

  async function toggleMaximize() {
    const appWindow = (await import('@tauri-apps/api/window')).appWindow
    appWindow.toggleMaximize()
  }

  async function toggleMinimize() {
    const appWindow = (await import('@tauri-apps/api/window')).appWindow
    appWindow.minimize()
  }

  return (
    <div data-tauri-drag-region className="bg-black text-white flex justify-between items-center">
      <div className="flex menu-options">
        <Record />
        <Serial />
      </div>
      <div className="flex px-4 space-x-2 gap-2">
        <button onClick={toggleMinimize} className="flex justify-center items-center">
          <FaMinus className="text-white w-3 h-3" />
        </button>
        <button onClick={toggleMaximize} className="flex justify-center items-center">
          <FaExpand className="text-white w-3 h-3" />
        </button>
        <button onClick={closeWindow} className="flex justify-center items-center">
          <FaTimes className="text-white w-4 h-4" />
        </button>
      </div>
    </div>
  );
}

