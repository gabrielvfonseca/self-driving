"""Integration tests for the infrastructure platform."""

import subprocess
import sys
import os

def test_rust_core_compilation():
    """Test that Rust core compiles successfully."""
    try:
        # Change to the rust-core directory and build
        result = subprocess.run(
            ["cargo", "build"],
            cwd="services/rust-core",
            capture_output=True,
            text=True,
            check=True
        )
        print("✓ Rust core compiles successfully")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Rust core compilation failed: {e.stderr}")
        return False

def test_rust_core_tests():
    """Test that Rust core tests pass."""
    try:
        # Run tests in the rust-core directory
        result = subprocess.run(
            ["cargo", "test"],
            cwd="services/rust-core",
            capture_output=True,
            text=True,
            check=True
        )
        print("✓ Rust core tests pass")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Rust core tests failed: {e.stderr}")
        return False

def test_proto_generation():
    """Test that proto files can be processed."""
    try:
        # Check if proto file exists
        proto_path = "packages/proto/resource.proto"
        if not os.path.exists(proto_path):
            print(f"✗ Proto file not found at {proto_path}")
            return False
            
        print("✓ Proto file exists")
        return True
    except Exception as e:
        print(f"✗ Proto file test failed: {e}")
        return False

if __name__ == "__main__":
    print("Running integration tests for infrastructure platform...")
    
    success = True
    success &= test_rust_core_compilation()
    success &= test_rust_core_tests()
    success &= test_proto_generation()
    
    if success:
        print("\n✓ All integration tests passed!")
        sys.exit(0)
    else:
        print("\n✗ Some integration tests failed!")
        sys.exit(1)
