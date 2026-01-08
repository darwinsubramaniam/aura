import { Sidebar as PrimeSidebar } from 'primereact/sidebar';

interface SidebarProps {
    visible: boolean;
    onHide: () => void;
}

export default function Sidebar({ visible, onHide }: SidebarProps) {

    return (
        <PrimeSidebar visible={visible} onHide={() => onHide()} className="w-full md:w-20rem lg:w-30rem">
            <h2>Sidebar</h2>
            <p>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
            </p>
        </PrimeSidebar>
    );
}