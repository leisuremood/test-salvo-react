import { useState } from "react";
import { LoadingOutlined, PlusOutlined } from "@ant-design/icons";
import { message, Upload } from "antd";

const baseURL = "http://localhost:3000/";
const getBase64 = (img, callback) => {
    const reader = new FileReader();
    reader.addEventListener("load", () => callback(reader.result));
    reader.readAsDataURL(img);
};

const beforeUpload = (file) => {
    const isJpgOrPng = file.type === "image/jpeg" || file.type === "image/png";
    if (!isJpgOrPng) {
        message.error("只能上传 JPG/PNG 文件!");
    }
    const isLt2M = file.size / 1024 / 1024 < 2;
    if (!isLt2M) {
        message.error("大小不能超过 2MB!");
    }
    return isJpgOrPng && isLt2M;
};

export default function Avatar({ form }) {
    const [loading, setLoading] = useState(false);
    const [imageUrl, setImageUrl] = useState();
    // 监视form表单中的photo变化。
    const handleChange = (info) => {
        // 上传中
        if (info.file.status === "uploading") {
            setLoading(true);
            return;
        }
        // 上传结束
        if (info.file.status === "done") {
            console.log(info);
            // Get this url from response in real world.
            getBase64(info.file.originFileObj, (url) => {
                setLoading(false);
                setImageUrl(url);
                console.log("@handle upload ", url);
            });
        }
    };

    const uploadButton = (
        <div>
            {loading ? <LoadingOutlined /> : <PlusOutlined />}
            <div
        style={{
                    marginTop: 38,
                }}
            >
                Upload
            </div>
        </div>
    );

    return (
        <>
            <Upload
                name="photo"
                listType="picture-card"
                className="avatar-uploader"
                showUploadList={false}
                action={baseURL + "admin/upload"}
                beforeUpload={beforeUpload}
                onChange={handleChange}
            >
                {imageUrl
                    ? (
                        <img
                            src={imageUrl}
                            alt="avatar"
                            style={{
                                width: "100%",
                            }}
                        />
                    )
                    : (
                        uploadButton
                    )}
            </Upload>
        </>
    );
}
