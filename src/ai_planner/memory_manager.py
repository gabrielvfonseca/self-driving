"""Memory management for AI planner agents."""

from typing import Dict, Any, List
import logging

logger = logging.getLogger(__name__)

class MemoryManager:
    """Manages agent memory and context."""
    
    def __init__(self):
        """Initialize the memory manager."""
        self.name = "Memory Manager"
        logger.info(f"Initialized {self.name}")
        self.memory = {}
    
    def store_context(self, key: str, data: Dict[str, Any]) -> None:
        """Store context data in memory."""
        logger.info(f"Storing context for {key}")
        self.memory[key] = data
    
    def retrieve_context(self, key: str) -> Dict[str, Any]:
        """Retrieve context data from memory."""
        logger.info(f"Retrieving context for {key}")
        return self.memory.get(key, {})
    
    def get_recent_decisions(self, limit: int = 10) -> List[Dict[str, Any]]:
        """Get recent decisions from memory."""
        logger.info("Retrieving recent decisions")
        # Mock implementation - in reality this would return actual stored decisions
        return []
