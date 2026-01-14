import { useNavigate } from 'react-router-dom';
import {
    Sheet,
    SheetContent,
    SheetHeader,
    SheetTitle,
    SheetDescription,
} from '@/components/ui/sheet';
import { Button } from '@/components/ui/button';
import { DollarSign, Settings } from 'lucide-react';

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
        <Sheet open={visible} onOpenChange={(open) => !open && onHide()}>
            <SheetContent side="left" className="w-full md:w-[20rem] lg:w-[30rem] pt-10">
                <SheetHeader className="mb-4 text-left">
                    <SheetTitle>Navigation</SheetTitle>
                    <SheetDescription className="sr-only">
                        Main application navigation
                    </SheetDescription>
                </SheetHeader>
                <div className="flex flex-col gap-2">
                    <Button
                        variant="ghost"
                        className="justify-start gap-2"
                        onClick={() => handleNavigate('/funding')}
                    >
                        <DollarSign className="h-4 w-4" />
                        Funding History
                    </Button>
                    <Button
                        variant="ghost"
                        className="justify-start gap-2"
                        onClick={() => handleNavigate('/user-settings')}
                    >
                        <Settings className="h-4 w-4" />
                        User Settings
                    </Button>
                </div>
            </SheetContent>
        </Sheet>
    );
}