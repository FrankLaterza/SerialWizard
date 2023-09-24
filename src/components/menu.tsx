import React from 'react';
import { Menu } from 'antd';
import {
  AppstoreOutlined,
  MailOutlined,
  SettingOutlined,
} from '@ant-design/icons';

const { SubMenu } = Menu;

function DesktopMenu() {
  return (
    <Menu mode="horizontal">
      <SubMenu key="FileMenu" icon={<SettingOutlined />} title="File">
        <Menu.Item key="files">Set Dir</Menu.Item>
        <Menu.Item key="record">Set Record</Menu.Item>
      </SubMenu>
      <SubMenu key="PortMenu" icon={<SettingOutlined />} title="Port">
        <Menu.Item key="connect">Connect</Menu.Item>
        <Menu.Item key="baud">Baud: </Menu.Item>
        <SubMenu key="PortsSubMenu" icon={<SettingOutlined />} title="Ports">
          <Menu.Item key="connect">Port 1</Menu.Item>
          <Menu.Item key="baud">Port 2</Menu.Item>
          <Menu.Item key="port">Port 3</Menu.Item>
          <Menu.Item key="setting:4">Port 4</Menu.Item>
        </SubMenu>
        <Menu.Item key="setting:4">Option 4</Menu.Item>
      </SubMenu>
    </Menu>
  );
}

export default DesktopMenu;
