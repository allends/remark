import React, { useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useEditable } from "use-editable";
import "./App.css";

function App() {
  const [text, setText] = React.useState("Starting point");
  const [markdown, setMarkdown] = React.useState("");

  const editorRef = useRef(null);
  useEditable(editorRef, setText);

  React.useEffect(() => {
    invoke("parse_md", { input: text }).then((result: any) => {
      setMarkdown(result);
    });
  }, [text]);

  return (
    <div className="App">
      <body>
        <div className="container">
          <textarea ref={editorRef}>{text}</textarea>
          <div dangerouslySetInnerHTML={{ __html: markdown }} />
        </div>
      </body>
    </div>
  );
}

export default App;
