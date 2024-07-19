using System.Net;
using System.Net.Sockets;

TcpListener myListener;

main();

void main() {
    myListener = new TcpListener(IPAddress.Parse("127.0.0.1"), 9999);
    myListener.Start();
    Console.WriteLine("Running on localhost:9999");
    StartListen();
}

void StartListen() {
    while (true) {
        TcpClient client = myListener.AcceptTcpClient();
        NetworkStream stream = client.GetStream();
        client.Close();
        HandleData();
        Console.WriteLine("Received Connection");
    }
}

