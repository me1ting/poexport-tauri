import Notify from "simple-notify";

export function notifyError(msg: string) {
    new Notify({
        status: "error",
        title: "",
        text: msg,
        effect: "fade",
        speed: 300,
        customClass: "",
        customIcon: "",
        showIcon: true,
        showCloseButton: true,
        autoclose: false,
        autotimeout: 3000,
        gap: 20,
        distance: 20,
        type: 3,
        position: "right top",
    });
}