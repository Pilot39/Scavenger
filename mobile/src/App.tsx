import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { createStackNavigator } from '@react-navigation/stack';
import { SafeAreaProvider } from 'react-native-safe-area-context';

import HomeScreen from './screens/HomeScreen';
import WasteSubmissionScreen from './screens/WasteSubmissionScreen';
import TransferScreen from './screens/TransferScreen';
import ProfileScreen from './screens/ProfileScreen';
import StatsScreen from './screens/StatsScreen';

const Tab = createBottomTabNavigator();
const Stack = createStackNavigator();

function HomeStack() {
  return (
    <Stack.Navigator>
      <Stack.Screen name="HomeMain" component={HomeScreen} options={{ title: 'Home' }} />
      <Stack.Screen name="Stats" component={StatsScreen} options={{ title: 'Statistics' }} />
    </Stack.Navigator>
  );
}

function WasteStack() {
  return (
    <Stack.Navigator>
      <Stack.Screen name="WasteMain" component={WasteSubmissionScreen} options={{ title: 'Submit Waste' }} />
      <Stack.Screen name="Transfer" component={TransferScreen} options={{ title: 'Transfer Waste' }} />
    </Stack.Navigator>
  );
}

export default function App() {
  return (
    <SafeAreaProvider>
      <NavigationContainer>
        <Tab.Navigator
          screenOptions={{
            headerShown: false,
            tabBarActiveTintColor: '#10b981',
            tabBarInactiveTintColor: '#6b7280',
          }}
        >
          <Tab.Screen
            name="Home"
            component={HomeStack}
            options={{
              tabBarLabel: 'Home',
              tabBarIcon: ({ color }) => <HomeIcon color={color} />,
            }}
          />
          <Tab.Screen
            name="Waste"
            component={WasteStack}
            options={{
              tabBarLabel: 'Waste',
              tabBarIcon: ({ color }) => <WasteIcon color={color} />,
            }}
          />
          <Tab.Screen
            name="Profile"
            component={ProfileScreen}
            options={{
              tabBarLabel: 'Profile',
              tabBarIcon: ({ color }) => <ProfileIcon color={color} />,
            }}
          />
        </Tab.Navigator>
      </NavigationContainer>
    </SafeAreaProvider>
  );
}

function HomeIcon({ color }: { color: string }) {
  return <Text style={{ color, fontSize: 24 }}>🏠</Text>;
}

function WasteIcon({ color }: { color: string }) {
  return <Text style={{ color, fontSize: 24 }}>♻️</Text>;
}

function ProfileIcon({ color }: { color: string }) {
  return <Text style={{ color, fontSize: 24 }}>👤</Text>;
}

import { Text } from 'react-native';
