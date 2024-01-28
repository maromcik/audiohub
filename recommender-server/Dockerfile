# Use a lightweight Python image
FROM python:3.8-slim

RUN apt-get update && apt-get install -y zip build-essential autoconf libtool pkg-config protobuf-compiler libprotobuf-dev

WORKDIR /app

COPY ./recommender_ai_grpc-master.zip /app

RUN unzip recommender_ai_grpc-master.zip && mv recommender_ai_grpc-master/* . && rm -r recommender_ai_grpc-master

# Install necessary dependencies
RUN python -m pip install virtualenv
RUN virtualenv venv
RUN /bin/bash -c "source venv/bin/activate && python -m pip install --upgrade pip"
RUN /bin/bash -c "source venv/bin/activate && python -m pip install grpcio grpcio-tools"
RUN /bin/bash -c "source venv/bin/activate && pip install transformers"
RUN /bin/bash -c "source venv/bin/activate && pip install torch==2.1.1+cpu -f https://download.pytorch.org/whl/torch_stable.html"
RUN /bin/bash -c "source venv/bin/activate && pip install numpy"

RUN /bin/bash -c "source venv/bin/activate && python -m grpc_tools.protoc -I./protos --python_out=. --pyi_out=. --grpc_python_out=. ./protos/recommender.proto"


CMD ["/bin/bash", "-c", "source venv/bin/activate && python server.py"]
