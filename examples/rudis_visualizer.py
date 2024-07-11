import redis
import gradio as gr

# 连接Redis的函数，接受地址、端口、密码和数据库编号作为参数
def connect_redis(host, port, password, db):
    try:
        return redis.Redis(host=host, port=int(port), password=password, db=int(db), decode_responses=True)
    except redis.ConnectionError as e:
        return str(e)

# 获取所有key的列表
def get_keys(redis_client):
    return list(redis_client.keys())

# 根据key获取其值
def get_value(redis_client, key):
    value = redis_client.get(key)
    if value is None:
        return f"Key '{key}' not found."
    return f"Key: {key}, Value: {value}"

def main(host, port, password, db, key=None):
    try:
        redis_client = connect_redis(host, port, password, db)
        if key:
            return get_value(redis_client, key)
        else:
            return get_keys(redis_client)
    except Exception as e:
        return str(e)

# 创建Gradio界面
iface = gr.Interface(
    fn=main,
    inputs=[
        gr.Textbox(label="Host", value="127.0.0.1", placeholder="Enter host"),
        gr.Number(label="Port", value=6379),
        gr.Textbox(label="Password", type="password", placeholder="Enter password"),
        gr.Number(label="DB", value=0), 
        gr.Textbox(label="Key", placeholder="Enter key to get its value")
    ],
    outputs="text", 
    title="Rudis Visualizer",
    description="Visualize Rudis keys and values."
)

# 启动Gradio应用
if __name__ == "__main__":
    iface.launch()