
import React from 'react';
import { Dropdown } from "@nextui-org/react";
import eta from "public/eta_space.png";
import { useState, useRef, useEffect } from "react";
import Image from "next/image";
import { FiSend, FiPlus, FiMinus } from "react-icons/fi";

interface DropDownButtonProps {
  isDropped: boolean;
  setIsDropped: React.Dispatch<React.SetStateAction<boolean>>;
}

function DropDownButton({ isDropped, setIsDropped }: DropDownButtonProps) {
  function handleDropToggle() {
    setIsDropped(!isDropped);
  }

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

interface HeaderProps {
  isDropped: boolean;
  setIsDropped: React.Dispatch<React.SetStateAction<boolean>>;
}

function Header({ isDropped, setIsDropped }: HeaderProps) {
  return (
    /* header */
    <div className="flex flex-row px-10 justify-center items-center w-full h-100 py-4 text-xl text-center bg-gray-900">
      <div className="flex justify-start w-1/3 gap-10">
        <DropDownButton isDropped={isDropped} setIsDropped={setIsDropped} />
      </div>
      <div className="flex justify-center w-1/3 text-center">Serial Monitor</div>
      <div className="flex justify-end w-1/3">
        <Image src={eta} width={150} height={150} alt="Picture of the author" />
      </div>
    </div>
  );
}

export default Header;
