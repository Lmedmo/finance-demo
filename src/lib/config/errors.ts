/* Types ----------------------------------------------------------------------------------------------- */
export interface ErrorMessage {
    id: number; 
    message: string
};

/* Default Values -------------------------------------------------------------------------------------- */
export const setEmptyErrorMsg: ErrorMessage = {
    id: 1, 
    message: "Invalid record type"
}
export const addRecErrorMsg: ErrorMessage = {
    id: 2, 
    message: "Failed to add record... RecordType not Addable"
}
export const editRecErrorMsg: ErrorMessage = {
    id: 3, 
    message: "Failed to edit record... RecordType not Editable"
}
export const deleteRecErrorMsg: ErrorMessage = {
    id: 4, 
    message: "Failed to delete record... RecordType not Deleteable"
}
export const missingLogicErrorMsg: ErrorMessage = {
    id: 5, 
    message: "Value failed to match a condition"
}
