import { ComponentChildren, JSX } from "preact";

export default function EditorWindow({ children }: { readonly children: ComponentChildren }): JSX.Element {
  return (
    <>
      { children }
    </>
  )
}
