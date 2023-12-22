<script lang="ts">
    import { Content } from '$lib/layouts';
    import { AppState, CurrentUser } from '$lib';
    import { Header, Setting, SettingVal, SettingBtn, SettingHeading, SettingGroup } from '$lib/components';
    import { getVersion } from '@tauri-apps/api/app';

    const appVersion = async () => await getVersion();
</script>

<Content>
    <Header src={$CurrentUser}/>
    <SettingGroup>
        <SettingHeading value="Appearence" />
        <Setting name="Theme">
            <div class="Options">
                {#key $AppState.themeval}
                <SettingBtn value='dark' state={$AppState.themeval} context='Theme' />
                <SettingBtn value='light' state={$AppState.themeval} context='Theme' />
                <SettingBtn value='Default' state={$AppState.themeval} context='Theme' />
                {/key}
            </div>
        </Setting>
        <Setting name="Font">
            <div class="Options">
                {#key $AppState.font}
                <SettingBtn value='Avenir' state={$AppState.font} context='Font' />
                <SettingBtn value='Avenir-Next' state={$AppState.font} context='Font' />
                <SettingBtn value='Quicksand' state={$AppState.font} context='Font' />
                {/key}
            </div>
        </Setting>
        <Setting name="Mode">
            <SettingVal value="Default" />
        </Setting>
    </SettingGroup>
    
    <SettingGroup>
        <SettingHeading value="Security" />
        <Setting name="Auth Required">
            <SettingVal value="No" />
        </Setting>
        <Setting name="Password">
            <SettingVal value={$CurrentUser.password} />
        </Setting>
        <Setting name="PIN">
            <SettingVal value={$CurrentUser.pin} />
        </Setting>
    </SettingGroup>
    
    <SettingGroup>
        <SettingHeading value="Advanced" />
        <Setting name="Data Storage Location">
            <SettingVal value="*Future*" />
        </Setting>
    </SettingGroup>
    
    <SettingGroup>
        <SettingHeading value="App Information" />
        <Setting name="Version">
            {#await appVersion() then ver}
            <SettingVal value={ver} />
            {/await}
        </Setting>
    </SettingGroup>
</Content>

<style>
    .Options{
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        gap: 5px;
    }
</style>