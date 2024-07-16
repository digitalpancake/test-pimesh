using System.Net;
using System.Net.Sockets;

TcpListener myListener;
int port = 5050;
IPAddress localAddr = IPAddress.Parse("127.0.0.1");

main();

//web server path
string WebServerPath = @"WebServer";

void main()
{
    myListener = new TcpListener(localAddr, port);
    myListener.Start();
    Console.WriteLine($"Web Server Running on {localAddr.ToString()} on port {port}... Press ^C to Stop...");
    StartListen();
}

void StartListen()
{
    while (true)
    {
        TcpClient client = myListener.AcceptTcpClient();
        NetworkStream stream = client.GetStream();
        client.Close();
    }
}
