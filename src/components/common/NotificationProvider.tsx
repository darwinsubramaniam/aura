import React, { createContext, useContext } from 'react';
import { toast } from "sonner";
import { Toaster } from "@/components/ui/sonner";
import { useIsMobile } from "@/hooks/use-mobile";

interface NotificationContextType {
    showSuccess: (message: string, summary?: string) => void;
    showError: (message: string, summary?: string) => void;
    showInfo: (message: string, summary?: string) => void;
    showWarn: (message: string, summary?: string) => void;
}

const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

export const NotificationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const isMobile = useIsMobile();

    const showSuccess = (message: string, summary: string = 'Success') => {
        toast.success(message, {
            description: summary,
        });
    };

    const showError = (message: string, summary: string = 'Error') => {
        toast.error(message, {
            description: summary,
        });
    };

    const showInfo = (message: string, summary: string = 'Info') => {
        toast.info(message, { description: summary });
    };

    const showWarn = (message: string, summary: string = 'Warning') => {
        toast.warning(message, { description: summary });
    };

    return (
        <NotificationContext.Provider value={{ showSuccess, showError, showInfo, showWarn }}>
            <Toaster
                position={isMobile ? "top-center" : "top-right"}
                closeButton={true}
                // Override the style directly to ensure top positioning respects the safe area
                style={isMobile ? {
                    top: 'max(60px, calc(env(safe-area-inset-top) + 20px))',
                    // Ensure it doesn't span full width blindly if we want it centered nicely
                    left: '52.5%',
                    transform: 'translateX(-50%)'
                } : undefined}
                // Keep offset as fallback/standard for desktop
                offset={isMobile ? undefined : "32px"}
            />
            {children}
        </NotificationContext.Provider>
    );
};

export const useNotification = () => {
    const context = useContext(NotificationContext);
    if (!context) {
        throw new Error('useNotification must be used within a NotificationProvider');
    }
    return context;
};
