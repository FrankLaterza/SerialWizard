import React from 'react';
import { FaFile, FaCog, FaMinus, FaExpand, FaTimes } from 'react-icons/fa';
import { useState, useRef, useEffect } from "react";

function MenuItem({ text }: any) {
  return <li className="cursor-pointer text-black">{text}</li>;
}

function SubMenu({ text, menuItemList }: any) {
  const [isSubOpen, setIsSubOpen] = useState(false);

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
      <span className="cursor-pointer text-black">{text}</span>
      {isSubOpen && (
        <ul className="absolute mx-2 space-y-4 bg-white p-2 left-full top-0">
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
      <ul className="flex justify-center items-center space-x-4">
        <li className="relative h-fit">
          <span className="cursor-pointer">Menu</span>
          {isDropdownOpen && (
            <ul className="flex flex-col w-max absolute space-y-2 bg-white p-2 block">
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

  const toggleDropdown = () => {
    setIsDropdownOpen(!isDropdownOpen);
  };

  const subMenuItems = [
    { text: "Hi" },
    { text: "How are you today?" },
    { text: "Hello" },
  ];

  return (
    <nav onMouseEnter={toggleDropdown} onMouseLeave={toggleDropdown}>
      <ul className="flex justify-center items-center space-x-4">
        <li className="relative">
          <span className="cursor-pointer">Menu</span>
          {isDropdownOpen && (
            <ul className="flex flex-col w-max absolute space-y-2 bg-white p-2 block">
              <MenuItem text="Hi" />
              <MenuItem text="Hello" />
              <SubMenu menuItemList={subMenuItems} />
              <SubMenu text={"submenu"} menuItemList={subMenuItems} />
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
    <div data-tauri-drag-region className="bg-black text-white m-2 flex justify-between items-center">
      <div className="flex space-x-6 menu-options">
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

