import React, { useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useEditable } from "use-editable";
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import "./App.css";

type FileMessage = {
  contents: string,
  path: string,
}

function App() {
  const [text, setText] = React.useState("");
  const [path, setPath] = React.useState("")
  const [markdown, setMarkdown] = React.useState("");

  appWindow.emit('event', { message: 'Tauri is awesome!' })

  React.useEffect(() => {
    invoke("parse_md", { input: text }).then((result: any) => {
      setMarkdown(result)
    })
  }, [text]);

  React.useEffect(() => {
    // loading the file that we have selected
    var file_contents: string = ""
    invoke("load_file").then((result: any) => {
      file_contents = result.contents
      setText(file_contents)
      setPath(result.path)
    })
  }, [])

  const editorRef = useRef(null);
  useEditable(editorRef, setText);

  return (
    <div className="App">
      <body>
        <div>{path}</div>
        <div className="container">
          <div ref={editorRef} className="textHolder">
            {text}
          </div>
          <div
            dangerouslySetInnerHTML={{ __html: markdown }}
            className="previewHolder"
          />
        </div>
      </body>
    </div>
  );
}

export default App;
