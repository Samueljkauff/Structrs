
export interface FolderNode {
  name: string;
  path: string;
  is_directory: boolean;
  parent: string | null;
  children: string[];
}