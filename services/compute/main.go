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

// ComputeService represents the compute service
type ComputeService struct {
    logger *logrus.Logger
}

// NewComputeService creates a new compute service
func NewComputeService() *ComputeService {
    return &ComputeService{
        logger: logrus.New(),
    }
}

// Start starts the compute service
func (s *ComputeService) Start(port string) error {
    lis, err := net.Listen("tcp", fmt.Sprintf(":%s", port))
    if err != nil {
        return fmt.Errorf("failed to listen: %v", err)
    }

    grpcServer := grpc.NewServer()
    
    // Register services here
    // grpcServer.RegisterService(&ComputeService{})
    
    // Register reflection service on gRPC server
    reflection.Register(grpcServer)

    s.logger.Infof("Starting compute service on port %s", port)
    
    // Graceful shutdown
    c := make(chan os.Signal, 1)
    signal.Notify(c, os.Interrupt, syscall.SIGTERM)
    
    go func() {
        if err := grpcServer.Serve(lis); err != nil {
            log.Fatalf("failed to serve: %v", err)
        }
    }()
    
    <-c
    s.logger.Info("Shutting down compute service...")
    grpcServer.GracefulStop()
    return nil
}

func main() {
    port := "8080"
    if len(os.Args) > 1 {
        port = os.Args[1]
    }
    
    service := NewComputeService()
    if err := service.Start(port); err != nil {
        log.Fatalf("failed to start service: %v", err)
    }
}
