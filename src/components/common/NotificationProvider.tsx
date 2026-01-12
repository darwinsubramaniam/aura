import React, { createContext, useContext } from 'react';
import { toast } from "sonner";
import { Toaster } from "@/components/ui/sonner";

interface NotificationContextType {
    showSuccess: (message: string, summary?: string) => void;
    showError: (message: string, summary?: string) => void;
    showInfo: (message: string, summary?: string) => void;
    showWarn: (message: string, summary?: string) => void;
}

const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

export const NotificationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {

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
            <Toaster position="top-right" closeButton={true} />
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
