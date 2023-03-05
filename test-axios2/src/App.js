import axios from "axios";
import Uploader from  "./Upload"
import "./App.css";

function App() {
    return (
        <div className="App">
            <button
                onClick={async () => {
                    const baseURL = "http://localhost:3000";
                    let instance = axios.create({
                        baseURL,
                        timeout: 8000,
                        // headers:{'X-Custom-Header': 'foobar'}
                    });
                    const account = { loginId: "admin", loginPwd: "123" };
                    const params = JSON.stringify(account);
                    const { data } = await axios.post("/admin/login", { data: params });
                    console.log("@", data);
                }}
            >
                test login
        </button>
            <Uploader/>
        </div>
    );
}

export default App;
