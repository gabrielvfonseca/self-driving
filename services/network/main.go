package main

import (
    "context"
    "fmt"
    "log"
    "net"
    "os"
    "os/signal"
    "syscall"

    "google.golang.org/grpc"
    "google.golang.org/grpc/reflection"
    
    "github.com/sirupsen/logrus"
)

// NetworkService represents the network service
type NetworkService struct {
    logger *logrus.Logger
}

// NewNetworkService creates a new network service
func NewNetworkService() *NetworkService {
    return &NetworkService{
        logger: logrus.New(),
    }
}

// Start starts the network service
func (s *NetworkService) Start(port string) error {
    lis, err := net.Listen("tcp", fmt.Sprintf(":%s", port))
    if err != nil {
        return fmt.Errorf("failed to listen: %v", err)
    }

    grpcServer := grpc.NewServer()
    
    // Register services here
    // grpcServer.RegisterService(&NetworkService{})
    
    // Register reflection service on gRPC server
    reflection.Register(grpcServer)

    s.logger.Infof("Starting network service on port %s", port)
    
    // Graceful shutdown
    c := make(chan os.Signal, 1)
    signal.Notify(c, os.Interrupt, syscall.SIGTERM)
    
    go func() {
        if err := grpcServer.Serve(lis); err != nil {
            log.Fatalf("failed to serve: %v", err)
        }
    }()
    
    <-c
    s.logger.Info("Shutting down network service...")
    grpcServer.GracefulStop()
    return nil
}

func main() {
    port := "8082"
    if len(os.Args) > 1 {
        port = os.Args[1]
    }
    
    service := NewNetworkService()
    if err := service.Start(port); err != nil {
        log.Fatalf("failed to start service: %v", err)
    }
}
