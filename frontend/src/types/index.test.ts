import { describe, it, expect } from 'vitest';
import {
  ParticipantRole,
  WasteType,
  WasteStatus,
  type Participant,
  type Waste,
  type Incentive,
  type ApiResponse,
} from './index';

describe('Type Definitions', () => {
  it('should define ParticipantRole enum correctly', () => {
    expect(ParticipantRole.Recycler).toBe(0);
    expect(ParticipantRole.Collector).toBe(1);
    expect(ParticipantRole.Manufacturer).toBe(2);
  });

  it('should define WasteType enum correctly', () => {
    expect(WasteType.Plastic).toBe('plastic');
    expect(WasteType.Paper).toBe('paper');
    expect(WasteType.Metal).toBe('metal');
  });

  it('should define WasteStatus enum correctly', () => {
    expect(WasteStatus.Submitted).toBe('submitted');
    expect(WasteStatus.Verified).toBe('verified');
    expect(WasteStatus.Transferred).toBe('transferred');
  });

  it('should allow creating valid Participant objects', () => {
    const participant: Participant = {
      address: '0x123',
      role: ParticipantRole.Recycler,
      name: 'John',
      latitude: 40.7128,
      longitude: -74.006,
      registeredAt: Date.now(),
    };
    expect(participant.role).toBe(ParticipantRole.Recycler);
  });

  it('should allow creating valid Waste objects', () => {
    const waste: Waste = {
      id: '1',
      type: WasteType.Plastic,
      weight: 100,
      owner: '0x123',
      latitude: 40.7128,
      longitude: -74.006,
      status: WasteStatus.Submitted,
      createdAt: Date.now(),
    };
    expect(waste.type).toBe(WasteType.Plastic);
  });

  it('should allow creating valid ApiResponse objects', () => {
    const response: ApiResponse<string> = {
      data: 'success',
      status: 200,
      message: 'Operation successful',
    };
    expect(response.status).toBe(200);
  });
});
