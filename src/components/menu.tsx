import React from 'react';
import { FaFile, FaCog, FaMinus, FaExpand, FaTimes } from 'react-icons/fa';
import { useState, useRef, useEffect } from "react";

function MenuItem({ text }: any) {
  return <li className="px-4 py-2 bg-white hover:bg-gray-400 cursor-pointer text-black">{text}</li>;
}

function SubMenu({ text, menuItemList }: any) {
  const [isSubOpen, setIsSubOpen] = useState(true);

  const openDropdown = () => {
    setIsSubOpen(true);
  };

  const closeDropdown = () => {
    setIsSubOpen(false);
  };

  return (
    <li
      className="relative"
      onMouseEnter={openDropdown}
      onMouseLeave={closeDropdown}
    >
      <li className="px-4 py-2 cursor-pointer text-black bg-white hover:bg-gray-400">{text}</li>
      {isSubOpen && (
        <ul className="absolute w-max bg-white left-full top-0">
          {menuItemList.map((item: any, index: any) => (
            <MenuItem key={index} text={item.text} />
          ))}
        </ul>
      )}
    </li>
  );
}

function Serial() {
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);

  const openDropdown = () => {
    setIsDropdownOpen(true);
  };

  const closeDropdown = () => {
    setIsDropdownOpen(false);
  };

  const subMenuItems = [
    { text: "Hi" },
    { text: "How are you today?" },
    { text: "Hello" },
  ];

  return (
    <nav
      onMouseEnter={openDropdown}
      onMouseLeave={closeDropdown}
    >
      <ul className="flex justify-center items-center py-2 cursor-pointer">
        <li className="relative h-fit px-4">
          <span >Serial</span>
          {isDropdownOpen && (
            <ul className="flex flex-col w-max absolute bg-white my-2 block">
              <MenuItem text="Hi" />
              <MenuItem text="Hello" />
              <SubMenu text={"Submenu"} menuItemList={subMenuItems} />
            </ul>
          )}
        </li>
      </ul>
    </nav>
  );
}

function Record(){ 
    const [isDropdownOpen, setIsDropdownOpen] = useState(false);

    const openDropdown = () => {
      setIsDropdownOpen(true);
    };
  
    const closeDropdown = () => {
      setIsDropdownOpen(false);
    };
  
    const subMenuItems = [
      { text: "Hi" },
      { text: "How are you today?" },
      { text: "Hello" },
    ];
  
    return (
      <nav
        onMouseEnter={openDropdown}
        onMouseLeave={closeDropdown}
      >
        <ul className="flex justify-center items-center py-2 cursor-pointer">
          <li className="relative h-fit px-4">
            <span >Menu</span>
            {isDropdownOpen && (
              <ul className="flex flex-col w-max absolute bg-white my-2 block">
                <MenuItem text="Hi" />
                <MenuItem text="Hello" />
                <SubMenu text={"Submenu"} menuItemList={subMenuItems} />
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
      <div className="flex space-x-2 gap-2">
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

