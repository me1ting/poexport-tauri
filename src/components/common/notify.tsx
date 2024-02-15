import { Toast, ToastIntent, ToastTitle } from "@fluentui/react-components";

export const TOASTER_ID = "uniqueToasterIdasdf1234";

export const notifySuccess = (controller: any, message: string) => {
    const { dispatchToast } = controller;
    const intent: ToastIntent = "success";
    dispatchToast(
        <Toast>
            <ToastTitle>{message}</ToastTitle>
        </Toast>,
        { intent }
    );
}

export const notifyError = (controller: any, message: string) => {
    const { dispatchToast } = controller;
    const intent: ToastIntent = "error";
    dispatchToast(
        <Toast>
            <ToastTitle>{message}</ToastTitle>
        </Toast>,
        { intent }
    );
}