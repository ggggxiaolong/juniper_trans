export default {
    btn: {
        bulkEdit: "Buld edit"
    },
    title: {
        personalInformation: "Personal Information",
        addLabel: "Add lable",
        morseSetup: "Setup morse-code",
        userCreateFail: "User createion failed"
    },
    message: {
        loginExpired: "Your login has expired, so you can cancel staying on the page or log in again",
        unkownError: "error",
        deleteLockConfirm: "Are you sure you want to delete this lock ?",
        deleteUserConfirm: "Are you sure you want to delete this user ?",
        deleteFingerprintConfirm: "Are you sure you want to delete this fingerprint ?",
        deleteTempAuthTime: "Are you sure you want to remove temporary access (${time}) from `${targetName}`",
        deleteGroupConfirm: "Are you sure you want to delete this group ?",
        setDefaultCardConfirm: "Are you sure you want to set this card as the default payment card ?"
    },
    tableColumn: {
        lastSyncTime: "Last sync time",
        tax: "Tax($)",
        lockCount: "No.of locks",
        totalMoney: "Total($)",
        bluetooth: "Bluetooth"
    },
    label: {
        lastSyncLocation: "Last sync location"
    },
    formValidate: {
        lockNameNotEmpty: "The lock name cannot be empty"
    },
    placeholder: {
        inputVerificationCode: "input verification code"
    },
    permission: {
        mangeMorseCode: "Manage morse code",
        receiveBluetooth: "Receive bluetooth/connection notification"
    },
    errorCodes: {
        code_400015: "This Tapplock Box is being used by the company, you cannot delete it",
        code_400046: "An error occurred while processing your card. Try again in a little bit."
    },
    auditReport: {
        authBluetooth: "#{operatorName} (#{operatorMail}) authorized #{userName} (#{mail}) permanent bluetooth access to #{lockName}",
        cancelBluetooth: "#{operatorName} (#{operatorMail}) revoked #{userName} (#{mail}) permanent bluetooth access to #{lockName}",
        authTemp: "#{operatorName} (#{operatorMail}) authorized #{userName} (#{mail}) temporary bluetooth access (#{times}) to #{lockName}",
        cancelTemp: "#{operatorName} (#{operatorMail}) revoked  #{userName} (#{mail}) temporary bluetooth access (#{time}) to #{lockName}",
        cancelAllTemp: "#{operatorName} (#{operatorMail}) revoked  #{userName} (#{mail}) all temporary bluetooth access to #{lockName}"
    },
    options: {
        addAuthBluetooth: "Authorized bluetooth access",
        deleteAuthBluetooth: "Revoked bluetooth access",
        setMorseCode: "Created Morse-code",
        activities: "User activities"
    },
    advanceAuditReport: {
        authNameBluetooth: "Bluetooth",
        addAuthBluetoothBatch: "#{userName} (#{mail}) authorized #{targetUserName} (#{targetUserMail}) permanent bluetooth access to #{targetLockName} through bulk authorization"
    }
}