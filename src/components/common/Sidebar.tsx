import { Sidebar as PrimeSidebar } from 'primereact/sidebar';
import { Button } from 'primereact/button';
import { useNavigate } from 'react-router-dom';

interface SidebarProps {
    visible: boolean;
    onHide: () => void;
}

export default function Sidebar({ visible, onHide }: SidebarProps) {
    const navigate = useNavigate();

    const handleNavigate = (path: string) => {
        navigate(path);
        onHide();
    };

    return (
        <PrimeSidebar visible={visible} onHide={onHide} className="w-full md:w-20rem lg:w-30rem">
            <div className="flex flex-col gap-2">
                <h2 className="text-xl font-bold mb-4 px-3">Navigation</h2>
                <Button
                    label="Funding History"
                    icon="pi pi-dollar"
                    className="p-button-text text-left justify-start"
                    onClick={() => handleNavigate('/funding')}
                />
                <Button
                    label="User Settings"
                    icon="pi pi-cog"
                    className="p-button-text text-left justify-start"
                    onClick={() => handleNavigate('/user-settings')}
                />
            </div>
        </PrimeSidebar>
    );
}