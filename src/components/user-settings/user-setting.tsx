import { invoke } from "@tauri-apps/api/core";
import { Card } from "primereact/card";
import { Dropdown } from "primereact/dropdown";
import { useEffect, useState } from "react";
import { UserSettings } from "./user-settings.modal";
import { Fiat } from "../funding/fiatramp.model";
import { Button } from "primereact/button";
import { useNotification } from "../common/NotificationProvider";

export default function UserSetting() {
    const { showSuccess, showError } = useNotification();
    const [userSettings, setUserSettings] = useState<UserSettings | null>(null);
    const [fiats, setFiats] = useState<Fiat[]>([]);
    const [oldUserSettings, setOldUserSettings] = useState<UserSettings | null>(null);

    const loadUserSettings = async () => {
        const userSettings = await invoke<UserSettings>('get_user_settings');
        setUserSettings(userSettings);
    }

    const loadAllFiats = async () => {
        const fiats = await invoke<Fiat[]>('get_all_fiat');
        setFiats(fiats);
    }

    useEffect(() => {
        loadUserSettings();
        loadAllFiats();
    }, []);


    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (userSettings && oldUserSettings) {
            await invoke<UserSettings>('update_user_settings', {
                userSettings: {
                    locale: userSettings?.locale,
                    default_fiat_id: userSettings?.default_fiat_id,
                }
            }).then((updatedUserSettings: UserSettings) => {
                console.log(`updatedUserSettings ${JSON.stringify(updatedUserSettings)}`);
                if (updatedUserSettings) {
                    setUserSettings(updatedUserSettings);
                }
                showSuccess('User settings updated successfully');
            }).catch((error) => {
                showError(`Failed to update user settings: ${error}`);
            });
        }

        setOldUserSettings(null);

    }
    return (
        <Card title="User Settings">
            <form onSubmit={handleSubmit} className="flex flex-col gap-4 p-4 max-w-lg mx-auto bg-base-100 shadow-xl rounded-box">
                <div className="form-control w-full">
                    <label className="label" htmlFor="name">
                        <span className="label-text">Currency</span>
                    </label>
                    <Dropdown
                        id="currency"
                        value={userSettings?.default_fiat_id}
                        onChange={(e) => {
                            if (userSettings) {
                                setOldUserSettings(userSettings);
                                setUserSettings({ ...userSettings, default_fiat_id: e.value })
                            }
                        }}
                        options={fiats}
                        optionLabel="name"
                        optionValue="id"
                        className="w-full"
                    />
                </div>
                <div className="flex justify-end">
                    <Button type="submit" label="Save" icon="pi pi-save" />
                </div>
            </form>
        </Card>
    );
}
