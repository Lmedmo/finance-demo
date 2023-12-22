import type { RecordType } from "./form";

/* Types ------------------------------------------------------------------------------------------------------ */
export interface ToolbarButtonConfig {
    action: ToolbarAction;
    type: RecordType;
}
export type ToolbarAction = "Add" | "Delete" | "Edit" | "Import" ;

/* Functions -------------------------------------------------------------------------------------------------- */
