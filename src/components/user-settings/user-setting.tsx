import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useNotification } from "../common/NotificationProvider";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Label } from "@/components/ui/label";
import { Save } from "lucide-react";
import { Fiat } from "@/lib/models/fiat";
import { FiatCommand } from "@/lib/services/fiat/fiat.command";

interface UserSettings {
    id: number;
    default_fiat_id: number;
    created_at: string;
    updated_at: string;
}

export default function UserSettings() {
    const { showSuccess, showError } = useNotification();
    const [fiats, setFiats] = useState<Fiat[]>([]);
    const [userSettings, setUserSettings] = useState<UserSettings>({ id: 0, default_fiat_id: 0, created_at: '', updated_at: '' });

    const loadAllFiats = async () => {
        const fiats = await FiatCommand.getAllCurrencies();
        setFiats(fiats);
    }

    const loadUserSettings = async () => {
        const settings = await invoke<UserSettings>('get_user_settings');
        setUserSettings(settings);
    }
    useEffect(() => {
        loadAllFiats();
        loadUserSettings();
    }, []);

    const updateUserSettings = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            await invoke('update_user_settings', { userSettings: { default_fiat_id: userSettings.default_fiat_id } });
            showSuccess('User settings updated successfully');
        } catch (error) {
            showError(`Failed to update user settings: ${error}`);
        }
    }


    return (
        <Card>
            <CardHeader>
                <CardTitle>User Settings</CardTitle>
            </CardHeader>
            <CardContent>
                <form onSubmit={updateUserSettings} className="space-y-4">
                    <div className="grid w-full max-w-sm items-center gap-1.5">
                        <Label htmlFor="defaultFiat">Default Fiat</Label>
                        <Select
                            value={userSettings.default_fiat_id.toString()}
                            onValueChange={(val) => setUserSettings({ ...userSettings, default_fiat_id: parseInt(val) })}
                        >
                            <SelectTrigger id="defaultFiat">
                                <SelectValue placeholder="Select default fiat" />
                            </SelectTrigger>
                            <SelectContent>
                                {fiats.map((fiat) => (
                                    <SelectItem key={fiat.id} value={fiat.id.toString()}>
                                        {fiat.name} ({fiat.symbol})
                                    </SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>

                    <div className="flex justify-end">
                        <Button type="submit">
                            <Save className="mr-2 h-4 w-4" /> Save
                        </Button>
                    </div>
                </form>
            </CardContent>
        </Card>
    );
}
